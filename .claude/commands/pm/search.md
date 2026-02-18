---
description: Search across all PRDs, epics, and tasks for matching content
argument-hint: <search-term>
allowed-tools: ListFiles, Read
---

Search across all project management content:

## Search Results
===============

### Search Term: $ARGUMENTS

### Search Scope:
Search across the following directories for the term "$ARGUMENTS":

1. **PRDs** - `.project/prds/` directory:
   - Search within all PRD markdown files
   - Match against titles, descriptions, and content

2. **Epics** - `.project/epics/` directory:
   - Search within epic directories
   - Match against epic titles, descriptions, and metadata

3. **Tasks** - `.project/epics/*/` directories:
   - Search within task files (matching `[0-9]*.md`)
   - Match against task titles, descriptions, and content

### Display Format:
For each match found, display:
- File type (PRD, Epic, or Task)
- File path
- Matching context (line or section containing the match)
- Brief excerpt showing the match

### Summary:
Provide a count of:
- Total matches found
- Matches by type (PRDs, Epics, Tasks)

### If No Matches Found:
If no matches are found for "$ARGUMENTS", display a message indicating no results were found.

Note: This command currently provides a manual overview. The automated search functionality will be available after scripts are migrated from `mirror/scripts/pm/search.sh` to `.claude/scripts/pm/search.sh`.
