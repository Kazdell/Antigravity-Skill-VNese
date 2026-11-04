#!/usr/bin/env node
/**
 * Workspace PreToolUse Hook (Language-Agnostic):
 * 1. For `edit_file`: Auto-normalizes LF -> CRLF in Target & Replacement when the file on disk uses CRLF,
 *    so multi-line `edit_file` calls succeed on the first try without line-ending mismatches.
 * 2. For `write_file`: Preserves the existing file's CRLF/LF line endings and allows overwriting
 *    ONLY when the surrounding file content is preserved (>= 50% unchanged lines), blocking full-file
 *    rewrites or accidental snippet truncations.
 */
const fs = require('fs');
const path = require('path');

let rawInput = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => {
  rawInput += chunk;
});

function toLfLines(text) {
  return text.replace(/\r\n/g, '\n').split('\n');
}

function countPreservedLines(oldLines, newLines) {
  const newBag = new Map();
  for (const line of newLines) {
    newBag.set(line, (newBag.get(line) || 0) + 1);
  }
  let matched = 0;
  for (const line of oldLines) {
    const count = newBag.get(line) || 0;
    if (count > 0) {
      matched++;
      newBag.set(line, count - 1);
    }
  }
  return matched;
}

process.stdin.on('end', () => {
  try {
    const payload = JSON.parse(rawInput || '{}');
    const toolName = (payload.toolCall && payload.toolCall.name) || '';
    const args = (payload.toolCall && payload.toolCall.args) || {};
    const targetPath = args.Path || args.path || '';

    if (!targetPath || !fs.existsSync(targetPath)) {
      process.stdout.write(JSON.stringify({ decision: 'allow' }));
      return;
    }

    const normalizedTarget = path.resolve(targetPath).toLowerCase().replace(/\\/g, '/');
    const artifactDir = (payload.artifactDirectoryPath || '').toLowerCase().replace(/\\/g, '/');

    if (
      (artifactDir && normalizedTarget.startsWith(artifactDir)) ||
      normalizedTarget.includes('/.gemini/antigravity/brain/')
    ) {
      process.stdout.write(JSON.stringify({ decision: 'allow' }));
      return;
    }

    const existingContent = fs.readFileSync(targetPath, 'utf8');
    const usesCrlf = existingContent.includes('\r\n');

    // 1. Handle edit_file: Auto-normalize CRLF (including mixed CRLF/LF files) in Target & Replacement
    if (toolName === 'edit_file') {
      const target = args.Target;
      const replacement = args.Replacement;
      if (usesCrlf && typeof target === 'string' && !existingContent.includes(target) && target.includes('\n')) {
        const crlfTarget = target.replace(/\r?\n/g, '\r\n');
        let matchedTarget = existingContent.includes(crlfTarget) ? crlfTarget : null;
        if (!matchedTarget) {
          const escaped = target
            .replace(/\r\n/g, '\n')
            .split('\n')
            .map((line) => line.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))
            .join('\\r?\\n');
          const match = existingContent.match(new RegExp(escaped));
          if (match) {
            matchedTarget = match[0];
          }
        }
        if (matchedTarget) {
          const crlfReplacement =
            typeof replacement === 'string'
              ? replacement.replace(/\r?\n/g, '\r\n')
              : replacement;
          process.stdout.write(
            JSON.stringify({
              decision: 'allow',
              overwrite: {
                Target: matchedTarget,
                Replacement: crlfReplacement
              }
            })
          );
          return;
        }
      }
      process.stdout.write(JSON.stringify({ decision: 'allow' }));
      return;
    }

    // 2. Handle write_file: Allow append, or allow surgical overwrite if >= 50% lines preserved & normalize CRLF
    const isAppend = args.Append === true || args.append === true;
    if (isAppend) {
      process.stdout.write(JSON.stringify({ decision: 'allow' }));
      return;
    }

    const newContent = typeof args.Content === 'string' ? args.Content : '';
    const oldLines = toLfLines(existingContent);
    const newLines = toLfLines(newContent);

    if (oldLines.length > 20) {
      const preserved = countPreservedLines(oldLines, newLines);
      const preservedRatio = preserved / oldLines.length;
      if (preservedRatio < 0.5) {
        process.stdout.write(
          JSON.stringify({
            decision: 'deny',
            reason:
              'BLOCKED BY WORKSPACE HOOK: write_file on ' +
              targetPath +
              ' only preserved ' +
              (preservedRatio * 100).toFixed(0) +
              '% of existing lines (< 50%). Do NOT rewrite the entire file or write a partial snippet; preserve all untouched lines and only modify the target block, or use edit_file.'
          })
        );
        return;
      }
    }

    const normalizedContent = usesCrlf
      ? newContent.replace(/\r?\n/g, '\r\n')
      : newContent.replace(/\r\n/g, '\n');

    if (normalizedContent !== newContent) {
      process.stdout.write(
        JSON.stringify({
          decision: 'allow',
          overwrite: {
            Content: normalizedContent
          }
        })
      );
      return;
    }

    process.stdout.write(JSON.stringify({ decision: 'allow' }));
  } catch (err) {
    process.stdout.write(JSON.stringify({ decision: 'allow' }));
  }
});
