import { useState, useCallback, useRef, useEffect } from 'react';
import { Transcript, Summary } from '@/types';
import { BlockNoteSummaryViewRef } from '@/components/AISummary/BlockNoteSummaryView';
import { CurrentMeeting, useSidebar } from '@/components/Sidebar/SidebarProvider';
import { invoke as invokeTauri } from '@tauri-apps/api/core';
import { toast } from 'sonner';

// Convert legacy summary format to markdown for file saving
function convertLegacySummaryToMarkdown(summary: Summary): string {
  const lines: string[] = [];
  for (const [, section] of Object.entries(summary)) {
    if (section && typeof section === 'object' && 'title' in section) {
      lines.push(`## ${section.title}\n`);
      if (Array.isArray(section.blocks)) {
        for (const block of section.blocks) {
          if (block?.content) {
            lines.push(`- ${block.content}`);
          }
        }
      }
      lines.push('');
    }
  }
  return lines.join('\n');
}

interface UseMeetingDataProps {
  meeting: any;
  summaryData: Summary | null;
  onMeetingUpdated?: () => Promise<void>;
}

export function useMeetingData({ meeting, summaryData, onMeetingUpdated }: UseMeetingDataProps) {
  // State
  // Use prop directly since summary generation fetches transcripts independently
  const transcripts = meeting.transcripts;
  const [meetingTitle, setMeetingTitle] = useState(meeting.title || '+ New Call');
  const [isEditingTitle, setIsEditingTitle] = useState(false);
  const [isTitleDirty, setIsTitleDirty] = useState(false);
  const [aiSummary, setAiSummary] = useState<Summary | null>(summaryData);
  const [isSaving, setIsSaving] = useState(false);
  const [, setIsSummaryDirty] = useState(false);
  const [, setError] = useState<string>('');

  // Ref for BlockNoteSummaryView
  const blockNoteSummaryRef = useRef<BlockNoteSummaryViewRef>(null);

  // Sidebar context
  const { setCurrentMeeting, setMeetings, meetings: sidebarMeetings } = useSidebar();

  // Sync aiSummary state when summaryData prop changes (fixes display of fetched summaries)
  useEffect(() => {
    console.log('[useMeetingData] Syncing summary data from prop:', summaryData ? 'present' : 'null');
    setAiSummary(summaryData);
  }, [summaryData]); // Only trigger when parent prop changes, not when aiSummary changes

  // Handlers
  const handleTitleChange = useCallback((newTitle: string) => {
    setMeetingTitle(newTitle);
    setIsTitleDirty(true);
  }, []);

  const handleSummaryChange = useCallback((newSummary: Summary) => {
    setAiSummary(newSummary);
  }, []);

  const handleSaveMeetingTitle = useCallback(async () => {
    try {
      await invokeTauri('api_save_meeting_title', {
        meetingId: meeting.id,
        title: meetingTitle,
      });

      console.log('Save meeting title success');
      setIsTitleDirty(false);

      // Update meetings with new title
      const updatedMeetings = sidebarMeetings.map((m: CurrentMeeting) =>
        m.id === meeting.id ? { id: m.id, title: meetingTitle } : m
      );
      setMeetings(updatedMeetings);
      setCurrentMeeting({ id: meeting.id, title: meetingTitle });
      return true;
    } catch (error) {
      console.error('Failed to save meeting title:', error);
      if (error instanceof Error) {
        setError(error.message);
      } else {
        setError('Failed to save meeting title: Unknown error');
      }
      return false;
    }
  }, [meeting.id, meetingTitle, sidebarMeetings, setMeetings, setCurrentMeeting]);

  const handleSaveSummary = useCallback(async (summary: Summary | { markdown?: string; summary_json?: any[] }) => {
    console.log('📄 handleSaveSummary called with:', {
      hasMarkdown: 'markdown' in summary,
      hasSummaryJson: 'summary_json' in summary,
      summaryKeys: Object.keys(summary)
    });

    try {
      let formattedSummary: any;
      let markdownContent: string | undefined;

      // Check if it's the new BlockNote format
      if ('markdown' in summary || 'summary_json' in summary) {
        console.log('📄 Saving new format (markdown/blocknote)');
        formattedSummary = summary;
        markdownContent = (summary as { markdown?: string }).markdown;
      } else {
        console.log('📄 Saving legacy format');
        formattedSummary = {
          MeetingName: meetingTitle,
          MeetingNotes: {
            sections: Object.entries(summary).map(([, section]) => ({
              title: section.title,
              blocks: section.blocks
            }))
          }
        };
        // Convert legacy format to markdown for file saving
        markdownContent = convertLegacySummaryToMarkdown(summary as Summary);
        console.log('📄 Converted legacy summary to markdown:', markdownContent.length, 'chars');
      }

      // Save to SQLite database
      await invokeTauri('api_save_meeting_summary', {
        meetingId: meeting.id,
        summary: formattedSummary,
      });

      console.log('✅ Save meeting summary to database success');

      // Also save to markdown file in recordings folder if folder_path exists
      if (meeting.folder_path && markdownContent) {
        try {
          // Pass the folder path - Rust will find the .md file in the folder
          const folderPath = meeting.folder_path.replace(/[/\\]$/, ''); // Remove trailing slash

          console.log('📝 Saving summary to markdown file in folder:', folderPath);

          await invokeTauri('update_meeting_summary', {
            path: folderPath,
            summary: markdownContent,
          });

          console.log('✅ Save meeting summary to markdown file success');
        } catch (mdError) {
          // Don't fail the whole save if markdown update fails
          console.warn('⚠️ Failed to save summary to markdown file:', mdError);
        }
      } else {
        console.log('ℹ️ Skipping markdown file save:', {
          hasFolderPath: !!meeting.folder_path,
          hasMarkdownContent: !!markdownContent
        });
      }
    } catch (error) {
      console.error('❌ Failed to save meeting summary:', error);
      if (error instanceof Error) {
        setError(error.message);
      } else {
        setError('Failed to save meeting summary: Unknown error');
      }
    }
  }, [meeting.id, meeting.folder_path, meetingTitle]);

  const saveAllChanges = useCallback(async () => {
    setIsSaving(true);
    try {
      // Save meeting title only if changed
      if (isTitleDirty) {
        await handleSaveMeetingTitle();
      }

      // Save BlockNote editor changes if dirty
      if (blockNoteSummaryRef.current?.isDirty) {
        console.log('💾 Saving BlockNote editor changes...');
        await blockNoteSummaryRef.current.saveSummary();
      } else if (aiSummary) {
        await handleSaveSummary(aiSummary);
      }

      toast.success("Changes saved successfully");
    } catch (error) {
      console.error('Failed to save changes:', error);
      toast.error("Failed to save changes", { description: String(error) });
    } finally {
      setIsSaving(false);
    }
  }, [isTitleDirty, handleSaveMeetingTitle, aiSummary, handleSaveSummary]);

  // Update meeting title from external source (e.g., AI summary)
  const updateMeetingTitle = useCallback((newTitle: string) => {
    console.log('📝 Updating meeting title to:', newTitle);
    setMeetingTitle(newTitle);
    const updatedMeetings = sidebarMeetings.map((m: CurrentMeeting) =>
      m.id === meeting.id ? { id: m.id, title: newTitle } : m
    );
    setMeetings(updatedMeetings);
    setCurrentMeeting({ id: meeting.id, title: newTitle });
  }, [meeting.id, sidebarMeetings, setMeetings, setCurrentMeeting]);

  return {
    // State
    transcripts,
    meetingTitle,
    isEditingTitle,
    isTitleDirty,
    aiSummary,
    isSaving,
    blockNoteSummaryRef,

    // Setters
    setMeetingTitle,
    setIsEditingTitle,
    setAiSummary,
    setIsSummaryDirty,

    // Handlers
    handleTitleChange,
    handleSummaryChange,
    handleSaveSummary,
    handleSaveMeetingTitle,
    saveAllChanges,
    updateMeetingTitle,
  };
}
