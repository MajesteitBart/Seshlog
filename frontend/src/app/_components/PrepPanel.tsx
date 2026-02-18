'use client';

import { useState } from 'react';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion';
import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import { Clock, FileText, Users, ListTodo, StickyNote, CheckSquare } from 'lucide-react';
import { format, parseISO } from 'date-fns';

/**
 * Meeting preparation data parsed from an Obsidian markdown file.
 * Mirrors the Rust MeetingPrep struct from src-tauri/src/obsidian/types.rs
 */
export interface MeetingPrep {
  file_path: string;
  frontmatter: MeetingFrontmatter;
  title: string;
  agenda: string[];
  notes: string[];
  action_items: string[];
  goals: string[];
  context: string;
  raw_content: string;
}

export interface MeetingFrontmatter {
  tags?: string | string[];
  date?: string;
  time_start?: string;
  time_end?: string;
  meeting_type?: string;
  attendees?: string[];
  status?: string;
}

interface PrepPanelProps {
  meetingPrep: MeetingPrep | null;
  isLoading?: boolean;
}

/**
 * Format a datetime string for display
 */
function formatDateTime(isoString: string): string {
  try {
    const date = parseISO(isoString);
    return format(date, 'MMM d, yyyy h:mm a');
  } catch {
    return isoString;
  }
}

/**
 * Format just the time portion of a datetime string
 */
function formatTime(isoString: string): string {
  try {
    const date = parseISO(isoString);
    return format(date, 'h:mm a');
  } catch {
    return isoString;
  }
}

/**
 * Check if an action item is marked as completed
 */
function isActionItemCompleted(item: string): boolean {
  return item.trim().startsWith('[x]') || item.trim().startsWith('[X]');
}

/**
 * Extract the text content from an action item, removing the checkbox prefix
 */
function getActionItemText(item: string): string {
  return item.replace(/^\s*\[[ xX]\]\s*/, '');
}

/**
 * Loading skeleton for the PrepPanel
 */
function PrepPanelSkeleton() {
  return (
    <div className="flex flex-col h-full p-4 space-y-4 animate-pulse">
      <div className="h-6 bg-gray-200 rounded w-3/4" />
      <div className="h-4 bg-gray-200 rounded w-1/2" />
      <div className="flex gap-2">
        <div className="h-6 bg-gray-200 rounded-full w-20" />
        <div className="h-6 bg-gray-200 rounded-full w-24" />
      </div>
      <div className="space-y-2">
        <div className="h-4 bg-gray-200 rounded w-full" />
        <div className="h-4 bg-gray-200 rounded w-5/6" />
        <div className="h-4 bg-gray-200 rounded w-4/6" />
      </div>
    </div>
  );
}

/**
 * Empty state shown when no meeting file is loaded
 */
function EmptyPrepState() {
  return (
    <div className="flex flex-col items-center justify-center h-full text-gray-500 p-8">
      <FileText className="w-12 h-12 mb-4 text-gray-400" />
      <p className="text-lg font-medium">No meeting file loaded</p>
      <p className="text-sm text-center mt-2">
        Open a meeting prep file to get started
      </p>
    </div>
  );
}

/**
 * PrepPanel Component
 *
 * Displays the parsed meeting prep content (Goals, Agenda, Context, Notes, Action Items)
 * in a read-only side panel. This shows the user their preparation notes during the meeting.
 */
export function PrepPanel({ meetingPrep, isLoading = false }: PrepPanelProps) {
  // Default open sections
  const [openSections, setOpenSections] = useState<string[]>([
    'agenda',
    'notes',
    'action-items',
  ]);

  if (isLoading) {
    return <PrepPanelSkeleton />;
  }

  if (!meetingPrep) {
    return <EmptyPrepState />;
  }

  const hasAgenda = meetingPrep.agenda.length > 0;
  const hasNotes = meetingPrep.notes.length > 0;
  const hasActionItems = meetingPrep.action_items.length > 0;
  const hasGoals = meetingPrep.goals.length > 0;
  const hasContext = meetingPrep.context.trim().length > 0;
  const hasAttendees = meetingPrep.frontmatter.attendees && meetingPrep.frontmatter.attendees.length > 0;

  return (
    <div className="flex flex-col h-full overflow-y-auto bg-white border-l border-gray-200">
      {/* Header section */}
      <div className="p-4 border-b border-gray-200">
        {/* Meeting Title */}
        <h2 className="text-lg font-semibold text-gray-900 mb-2">
          {meetingPrep.title || 'Untitled Meeting'}
        </h2>

        {/* Date/Time Info */}
        {meetingPrep.frontmatter.time_start && (
          <div className="flex items-center text-sm text-gray-600 mb-3">
            <Clock className="w-4 h-4 mr-2" />
            <span>
              {formatDateTime(meetingPrep.frontmatter.time_start)}
              {meetingPrep.frontmatter.time_end && (
                <> – {formatTime(meetingPrep.frontmatter.time_end)}</>
              )}
            </span>
          </div>
        )}

        {/* Attendees Badges */}
        {hasAttendees && (
          <div className="flex flex-wrap items-center gap-2">
            <Users className="w-4 h-4 text-gray-400" />
            {meetingPrep.frontmatter.attendees!.map((attendee, index) => (
              <Badge key={index} variant="secondary">
                {attendee}
              </Badge>
            ))}
          </div>
        )}
      </div>

      {/* Content sections */}
      <div className="flex-1 overflow-y-auto p-4">
        <Accordion
          type="multiple"
          value={openSections}
          onValueChange={setOpenSections}
          className="space-y-2"
        >
          {/* Agenda Section */}
          {hasAgenda && (
            <AccordionItem value="agenda" className="border rounded-lg px-4">
              <AccordionTrigger className="py-3">
                <div className="flex items-center gap-2">
                  <ListTodo className="w-4 h-4 text-gray-500" />
                  <span>Agenda</span>
                  <Badge variant="outline" className="ml-2">
                    {meetingPrep.agenda.length}
                  </Badge>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <ul className="space-y-2 pb-2">
                  {meetingPrep.agenda.map((item, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-2 text-sm text-gray-700"
                    >
                      <span className="text-gray-400 font-medium min-w-[20px]">
                        {index + 1}.
                      </span>
                      <span>{item}</span>
                    </li>
                  ))}
                </ul>
              </AccordionContent>
            </AccordionItem>
          )}

          {/* Notes Section */}
          {hasNotes && (
            <AccordionItem value="notes" className="border rounded-lg px-4">
              <AccordionTrigger className="py-3">
                <div className="flex items-center gap-2">
                  <StickyNote className="w-4 h-4 text-gray-500" />
                  <span>Notes</span>
                  <Badge variant="outline" className="ml-2">
                    {meetingPrep.notes.length}
                  </Badge>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <ul className="space-y-2 pb-2">
                  {meetingPrep.notes.map((note, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-2 text-sm text-gray-700"
                    >
                      <span className="text-gray-400">•</span>
                      <span>{note}</span>
                    </li>
                  ))}
                </ul>
              </AccordionContent>
            </AccordionItem>
          )}

          {/* Action Items Section */}
          {hasActionItems && (
            <AccordionItem value="action-items" className="border rounded-lg px-4">
              <AccordionTrigger className="py-3">
                <div className="flex items-center gap-2">
                  <CheckSquare className="w-4 h-4 text-gray-500" />
                  <span>Action Items</span>
                  <Badge variant="outline" className="ml-2">
                    {meetingPrep.action_items.length}
                  </Badge>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <ul className="space-y-3 pb-2">
                  {meetingPrep.action_items.map((item, index) => {
                    const isCompleted = isActionItemCompleted(item);
                    const text = getActionItemText(item);
                    return (
                      <li key={index} className="flex items-start gap-3">
                        <Checkbox
                          checked={isCompleted}
                          disabled
                          className="mt-0.5"
                        />
                        <span
                          className={`text-sm ${
                            isCompleted
                              ? 'text-gray-400 line-through'
                              : 'text-gray-700'
                          }`}
                        >
                          {text}
                        </span>
                      </li>
                    );
                  })}
                </ul>
              </AccordionContent>
            </AccordionItem>
          )}

          {/* Goals Section (legacy support) */}
          {hasGoals && (
            <AccordionItem value="goals" className="border rounded-lg px-4">
              <AccordionTrigger className="py-3">
                <div className="flex items-center gap-2">
                  <CheckSquare className="w-4 h-4 text-gray-500" />
                  <span>Goals</span>
                  <Badge variant="outline" className="ml-2">
                    {meetingPrep.goals.length}
                  </Badge>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <ul className="space-y-3 pb-2">
                  {meetingPrep.goals.map((goal, index) => {
                    const isCompleted = isActionItemCompleted(goal);
                    const text = getActionItemText(goal);
                    return (
                      <li key={index} className="flex items-start gap-3">
                        <Checkbox
                          checked={isCompleted}
                          disabled
                          className="mt-0.5"
                        />
                        <span
                          className={`text-sm ${
                            isCompleted
                              ? 'text-gray-400 line-through'
                              : 'text-gray-700'
                          }`}
                        >
                          {text}
                        </span>
                      </li>
                    );
                  })}
                </ul>
              </AccordionContent>
            </AccordionItem>
          )}

          {/* Context Section (legacy support) */}
          {hasContext && (
            <AccordionItem value="context" className="border rounded-lg px-4">
              <AccordionTrigger className="py-3">
                <div className="flex items-center gap-2">
                  <FileText className="w-4 h-4 text-gray-500" />
                  <span>Context</span>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                <div className="prose prose-sm max-w-none text-gray-700 pb-2">
                  {meetingPrep.context}
                </div>
              </AccordionContent>
            </AccordionItem>
          )}
        </Accordion>

        {/* Show message if no content sections */}
        {!hasAgenda && !hasNotes && !hasActionItems && !hasGoals && !hasContext && (
          <div className="text-center text-gray-500 py-8">
            <p>No preparation content found in this file.</p>
            <p className="text-sm mt-2">
              Add Agenda, Notes, or Action Items sections to your meeting file.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default PrepPanel;
