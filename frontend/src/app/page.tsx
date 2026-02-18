'use client';

import { useState, useEffect } from 'react';
import { RecordingControls } from '@/components/RecordingControls';
import { useSidebar } from '@/components/Sidebar/SidebarProvider';
import { usePermissionCheck } from '@/hooks/usePermissionCheck';
import { useRecordingState, RecordingStatus } from '@/contexts/RecordingStateContext';
import { useTranscripts } from '@/contexts/TranscriptContext';
import { useConfig } from '@/contexts/ConfigContext';
import { StatusOverlays } from '@/app/_components/StatusOverlays';
import Analytics from '@/lib/analytics';
import { SettingsModals } from './_components/SettingsModal';
import { TranscriptPanel } from './_components/TranscriptPanel';
import { PrepPanel, MeetingPrep } from './_components/PrepPanel';
import { useModalState } from '@/hooks/useModalState';
import { useRecordingStateSync } from '@/hooks/useRecordingStateSync';
import { useRecordingStart } from '@/hooks/useRecordingStart';
import { useRecordingStop } from '@/hooks/useRecordingStop';
import { useTranscriptRecovery } from '@/hooks/useTranscriptRecovery';
import { TranscriptRecovery } from '@/components/TranscriptRecovery';
import { indexedDBService } from '@/services/indexedDBService';
import { toast } from 'sonner';
import { useRouter } from 'next/navigation';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Button } from '@/components/ui/button';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from '@/components/ui/sheet';
import { FolderOpen, X, FileText } from 'lucide-react';

export default function Home() {
  // Local page state (not moved to contexts)
  const [isRecording, setIsRecordingState] = useState(false);
  const [barHeights, setBarHeights] = useState(['58%', '76%', '58%']);
  const [showRecoveryDialog, setShowRecoveryDialog] = useState(false);

  // Meeting prep state
  const [meetingPrep, setMeetingPrep] = useState<MeetingPrep | null>(null);
  const [isLoadingFile, setIsLoadingFile] = useState(false);
  const [isMobilePrepOpen, setIsMobilePrepOpen] = useState(false);

  // Use contexts for state management
  const { meetingTitle } = useTranscripts();
  const { transcriptModelConfig, selectedDevices } = useConfig();
  const recordingState = useRecordingState();

  // Extract status from global state
  const { status, isStopping, isProcessing, isSaving } = recordingState;

  // Hooks
  const { hasMicrophone } = usePermissionCheck();
  const { setIsMeetingActive, refetchMeetings } = useSidebar();
  const { modals, messages, showModal, hideModal } = useModalState(transcriptModelConfig);
  const { isRecordingDisabled, setIsRecordingDisabled } = useRecordingStateSync(isRecording, setIsRecordingState, setIsMeetingActive);
  const { handleRecordingStart } = useRecordingStart(isRecording, setIsRecordingState, showModal);

  // Get handleRecordingStop function and setIsStopping (state comes from global context)
  const { handleRecordingStop, setIsStopping } = useRecordingStop(
    setIsRecordingState,
    setIsRecordingDisabled
  );

  // Recovery hook
  const {
    recoverableMeetings,
    isLoading: isLoadingRecovery,
    isRecovering,
    checkForRecoverableTranscripts,
    recoverMeeting,
    loadMeetingTranscripts,
    deleteRecoverableMeeting
  } = useTranscriptRecovery();

  const router = useRouter();

  // Handle opening meeting file
  const handleOpenFile = async () => {
    try {
      const selected = await open({
        filters: [{ name: 'Markdown', extensions: ['md'] }],
        multiple: false,
        directory: false,
      });

      if (selected && typeof selected === 'string') {
        setIsLoadingFile(true);
        try {
          const prep = await invoke<MeetingPrep>('open_meeting_file', { path: selected });
          setMeetingPrep(prep);
          toast.success('Meeting file loaded', {
            description: prep.title || 'Untitled Meeting',
          });
        } catch (error) {
          console.error('Failed to parse meeting file:', error);
          toast.error('Failed to load meeting file', {
            description: error instanceof Error ? error.message : 'Unknown error',
          });
        } finally {
          setIsLoadingFile(false);
        }
      }
    } catch (error) {
      console.error('Failed to open file dialog:', error);
      toast.error('Failed to open file picker');
    }
  };

  // Handle closing meeting file
  const handleCloseFile = () => {
    setMeetingPrep(null);
  };

  useEffect(() => {
    // Track page view
    Analytics.trackPageView('home');
  }, []);

  // Startup recovery check
  useEffect(() => {
    const performStartupChecks = async () => {
      try {
        // Skip recovery check if currently recording or processing stop
        // This prevents the recovery dialog from showing when:
        if (recordingState.isRecording ||
          status === RecordingStatus.STOPPING ||
          status === RecordingStatus.PROCESSING_TRANSCRIPTS ||
          status === RecordingStatus.SAVING) {
          console.log('Skipping recovery check - recording in progress or processing');
          return;
        }

        // 1. Clean up old meetings (7+ days)
        try {
          await indexedDBService.deleteOldMeetings(7);
        } catch (error) {
          console.warn('Failed to clean up old meetings:', error);
        }

        // 2. Clean up saved meetings (24+ hours after save)
        try {
          await indexedDBService.deleteSavedMeetings(24);
        } catch (error) {
          console.warn('Failed to clean up saved meetings:', error);
        }

        // 3. Always check for recoverable meetings on startup
        // Don't skip based on sessionStorage - we need to check every time
        await checkForRecoverableTranscripts();
      } catch (error) {
        console.error('Failed to perform startup checks:', error);
      }
    };

    performStartupChecks();
  }, [checkForRecoverableTranscripts, recordingState.isRecording, status]);

  // Watch for recoverable meetings changes and show dialog once per session
  useEffect(() => {
    // Only show dialog if we have meetings and haven't shown it yet this session
    if (recoverableMeetings.length > 0) {
      const shownThisSession = sessionStorage.getItem('recovery_dialog_shown');
      if (!shownThisSession) {
        setShowRecoveryDialog(true);
        sessionStorage.setItem('recovery_dialog_shown', 'true');
      }
    }
  }, [recoverableMeetings]);

  // Handle recovery with toast notifications and navigation
  const handleRecovery = async (meetingId: string) => {
    try {
      const result = await recoverMeeting(meetingId);

      if (result.success) {
        toast.success('Meeting recovered successfully!', {
          description: result.audioRecoveryStatus?.status === 'success'
            ? 'Transcripts and audio recovered'
            : 'Transcripts recovered (no audio available)',
          action: result.meetingId ? {
            label: 'View Meeting',
            onClick: () => {
              router.push(`/meeting-details?id=${result.meetingId}`);
            }
          } : undefined,
          duration: 10000,
        });

        // Refresh sidebar to show the newly recovered meeting
        await refetchMeetings();

        // If no more recoverable meetings, clear session flag so dialog can show again
        if (recoverableMeetings.length === 0) {
          sessionStorage.removeItem('recovery_dialog_shown');
        }

        // Auto-navigate after a short delay
        if (result.meetingId) {
          setTimeout(() => {
            router.push(`/meeting-details?id=${result.meetingId}`);
          }, 2000);
        }
      }
    } catch (error) {
      toast.error('Failed to recover meeting', {
        description: error instanceof Error ? error.message : 'Unknown error occurred',
      });
      throw error;
    }
  };

  // Handle dialog close - clear session flag if no meetings left
  const handleDialogClose = () => {
    setShowRecoveryDialog(false);
    // If user closes dialog and there are no more meetings, clear the flag
    // This allows the dialog to show again next session if new meetings appear
    if (recoverableMeetings.length === 0) {
      sessionStorage.removeItem('recovery_dialog_shown');
    }
  };

  useEffect(() => {
    if (recordingState.isRecording) {
      const interval = setInterval(() => {
        setBarHeights(prev => {
          const newHeights = [...prev];
          newHeights[0] = Math.random() * 20 + 10 + 'px';
          newHeights[1] = Math.random() * 20 + 10 + 'px';
          newHeights[2] = Math.random() * 20 + 10 + 'px';
          return newHeights;
        });
      }, 300);

      return () => clearInterval(interval);
    }
  }, [recordingState.isRecording]);

  // Computed values using global status
  const isProcessingStop = status === RecordingStatus.PROCESSING_TRANSCRIPTS || isProcessing;

  return (
    <div className="flex flex-col h-screen bg-gray-50">
      {/* All Modals supported*/}
      <SettingsModals
        modals={modals}
        messages={messages}
        onClose={hideModal}
      />

      {/* Recovery Dialog */}
      <TranscriptRecovery
        isOpen={showRecoveryDialog}
        onClose={handleDialogClose}
        recoverableMeetings={recoverableMeetings}
        onRecover={handleRecovery}
        onDelete={deleteRecoverableMeeting}
        onLoadPreview={loadMeetingTranscripts}
      />

      {/* Header with file picker */}
      <header className="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-white">
        <div className="flex items-center gap-3">
          {/* Mobile: Show prep panel toggle button when file is loaded */}
          {meetingPrep && (
            <Button
              variant="ghost"
              size="sm"
              className="md:hidden"
              onClick={() => setIsMobilePrepOpen(true)}
              title="Show meeting prep"
            >
              <FileText className="w-4 h-4" />
            </Button>
          )}

          <Button
            variant="outline"
            size="sm"
            onClick={handleOpenFile}
            disabled={isLoadingFile}
          >
            <FolderOpen className="w-4 h-4 mr-2" />
            {isLoadingFile ? 'Loading...' : 'Open Meeting File'}
          </Button>

          {meetingPrep && (
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <span className="max-w-[300px] truncate" title={meetingPrep.file_path}>
                {meetingPrep.title || meetingPrep.file_path.split(/[/\\]/).pop()}
              </span>
              <button
                onClick={handleCloseFile}
                className="p-1 hover:bg-gray-100 rounded"
                title="Close file"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
          )}
        </div>
      </header>

      {/* Mobile drawer for PrepPanel */}
      <Sheet open={isMobilePrepOpen} onOpenChange={setIsMobilePrepOpen}>
        <SheetContent side="left" className="w-[85vw] sm:w-[350px] p-0">
          <SheetHeader className="px-4 py-3 border-b">
            <SheetTitle>Meeting Prep</SheetTitle>
          </SheetHeader>
          <div className="h-[calc(100vh-60px)] overflow-y-auto">
            <PrepPanel meetingPrep={meetingPrep} isLoading={isLoadingFile} />
          </div>
        </SheetContent>
      </Sheet>

      {/* Two-Panel Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Prep Panel - 1/3 width on desktop, hidden on mobile */}
        <div className="hidden md:flex md:w-1/3 lg:w-1/4 border-r border-gray-200">
          <PrepPanel meetingPrep={meetingPrep} isLoading={isLoadingFile} />
        </div>

        {/* Transcript Panel - 2/3 width on desktop, full width on mobile */}
        <div className="flex-1">
          <TranscriptPanel
            isProcessingStop={isProcessingStop}
            isStopping={isStopping}
            showModal={showModal}
          />
        </div>
      </div>

      {/* Recording controls - only show when permissions are granted or already recording and not showing status messages */}
      {(hasMicrophone || isRecording) &&
        status !== RecordingStatus.PROCESSING_TRANSCRIPTS &&
        status !== RecordingStatus.SAVING && (
          <div className="fixed bottom-12 left-0 right-0 z-10">
            <div className="flex justify-center">
              <div className="w-2/3 max-w-[750px] flex justify-center">
                <div className="bg-white rounded-full shadow-lg flex items-center">
                  <RecordingControls
                    isRecording={recordingState.isRecording}
                    onRecordingStop={(callApi = true) => handleRecordingStop(callApi)}
                    onRecordingStart={handleRecordingStart}
                    onTranscriptReceived={() => { }} // Not actually used by RecordingControls
                    onStopInitiated={() => setIsStopping(true)}
                    barHeights={barHeights}
                    onTranscriptionError={(message) => {
                      showModal('errorAlert', message);
                    }}
                    isRecordingDisabled={isRecordingDisabled}
                    isParentProcessing={isProcessingStop}
                    selectedDevices={selectedDevices}
                    meetingName={meetingTitle}
                  />
                </div>
              </div>
            </div>
          </div>
        )}

      {/* Status Overlays - Processing and Saving */}
      <StatusOverlays
        isProcessing={status === RecordingStatus.PROCESSING_TRANSCRIPTS && !recordingState.isRecording}
        isSaving={status === RecordingStatus.SAVING}
      />
    </div>
  );
}
