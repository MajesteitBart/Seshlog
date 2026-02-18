'use client';

import { useState } from 'react';
import { PrepPanel, MeetingPrep } from '../_components/PrepPanel';
import { Button } from '@/components/ui/button';

/**
 * Sample meeting data for testing the PrepPanel component
 */
const sampleMeetingPrep: MeetingPrep = {
  file_path: '/Users/demo/Obsidian/Meetings/2026-01-23-project-kickoff.md',
  frontmatter: {
    tags: 'meeting',
    date: '2026-01-23',
    time_start: '2026-01-23T10:00:00',
    time_end: '2026-01-23T11:00:00',
    meeting_type: 'kickoff',
    attendees: ['Alice Chen', 'Bob Smith', 'Carol Johnson', 'David Lee'],
    status: 'scheduled',
  },
  title: 'Project Kickoff: Meeting Companion',
  agenda: [
    'Introductions and team roles (5 min)',
    'Review project scope and timeline (15 min)',
    'Technical architecture overview (20 min)',
    'Sprint planning for week 1 (15 min)',
    'Q&A and next steps (5 min)',
  ],
  notes: [
    'Team is excited about the Obsidian integration',
    'Need to clarify API key management approach',
    'Consider offline fallback for transcription',
  ],
  action_items: [
    '[ ] Set up development environment for all team members',
    '[x] Share Figma designs with the team',
    '[ ] Create GitHub issues for sprint 1 tasks',
    '[ ] Schedule follow-up meeting for architecture review',
  ],
  goals: [
    '[ ] Align on project timeline',
    '[x] Assign initial workstreams',
    '[ ] Define MVP scope',
  ],
  context:
    'This is the kickoff meeting for the Meeting Companion project, a fork of Meetily that focuses on Obsidian integration. The goal is to bridge meeting preparation in Obsidian with live transcription.',
  raw_content: '---\ntags: meeting\ndate: 2026-01-23\n---\n\n# Project Kickoff...',
};

/**
 * Minimal meeting data for testing edge cases
 */
const minimalMeetingPrep: MeetingPrep = {
  file_path: '/Users/demo/Obsidian/Meetings/quick-sync.md',
  frontmatter: {
    date: '2026-01-23',
    attendees: ['Alice'],
  },
  title: 'Quick Sync',
  agenda: ['Status update'],
  notes: [],
  action_items: [],
  goals: [],
  context: '',
  raw_content: '# Quick Sync\n\n## Agenda\n- Status update',
};

/**
 * Test page for the PrepPanel component
 */
export default function TestPrepPage() {
  const [currentPrep, setCurrentPrep] = useState<MeetingPrep | null>(sampleMeetingPrep);
  const [isLoading, setIsLoading] = useState(false);

  const simulateLoading = () => {
    setIsLoading(true);
    setCurrentPrep(null);
    setTimeout(() => {
      setCurrentPrep(sampleMeetingPrep);
      setIsLoading(false);
    }, 1500);
  };

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Control Panel */}
      <div className="w-64 bg-white border-r border-gray-200 p-4 space-y-4">
        <h1 className="text-lg font-bold">PrepPanel Test</h1>
        <p className="text-sm text-gray-500">
          Test the PrepPanel component with different states
        </p>

        <div className="space-y-2">
          <Button
            className="w-full"
            onClick={() => setCurrentPrep(sampleMeetingPrep)}
          >
            Full Sample
          </Button>
          <Button
            className="w-full"
            variant="outline"
            onClick={() => setCurrentPrep(minimalMeetingPrep)}
          >
            Minimal Sample
          </Button>
          <Button
            className="w-full"
            variant="outline"
            onClick={() => setCurrentPrep(null)}
          >
            Empty State
          </Button>
          <Button
            className="w-full"
            variant="outline"
            onClick={simulateLoading}
          >
            Simulate Loading
          </Button>
        </div>

        <div className="pt-4 border-t">
          <h2 className="text-sm font-medium mb-2">Current State</h2>
          <ul className="text-xs text-gray-600 space-y-1">
            <li>
              Data:{' '}
              {currentPrep
                ? currentPrep.title
                : isLoading
                ? 'Loading...'
                : 'None'}
            </li>
            <li>Loading: {isLoading ? 'Yes' : 'No'}</li>
          </ul>
        </div>
      </div>

      {/* PrepPanel Preview */}
      <div className="flex-1 flex">
        {/* Mock transcript area */}
        <div className="flex-1 bg-gray-50 p-4">
          <div className="bg-white rounded-lg shadow-sm p-4 h-full">
            <h2 className="text-lg font-medium text-gray-700 mb-4">
              Transcript Area (Mock)
            </h2>
            <div className="space-y-3 text-sm text-gray-600">
              <p>
                <span className="font-medium">[00:00:12] Alice:</span> Welcome
                everyone to the project kickoff meeting.
              </p>
              <p>
                <span className="font-medium">[00:00:28] Bob:</span> Thanks for
                having us. Excited to get started!
              </p>
              <p>
                <span className="font-medium">[00:00:45] Alice:</span> Let&apos;s
                start with quick introductions.
              </p>
              <p className="text-gray-400 italic">
                (This area simulates the transcript panel)
              </p>
            </div>
          </div>
        </div>

        {/* PrepPanel - 1/3 width on right */}
        <div className="w-80">
          <PrepPanel meetingPrep={currentPrep} isLoading={isLoading} />
        </div>
      </div>
    </div>
  );
}
