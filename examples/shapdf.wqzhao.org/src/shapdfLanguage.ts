import type * as Monaco from 'monaco-editor';

export const shapdfLanguageConfig: Monaco.languages.LanguageConfiguration = {
  comments: {
    lineComment: '#',
  },
  brackets: [
    ['(', ')'],
  ],
  autoClosingPairs: [
    { open: '(', close: ')' },
  ],
  surroundingPairs: [
    { open: '(', close: ')' },
  ],
};

export const shapdfTokensProvider: Monaco.languages.IMonarchLanguage = {
  defaultToken: '',
  ignoreCase: false,

  tokenizer: {
    root: [
      // Hex colors MUST come before comments (more specific pattern first)
      [/#[0-9a-fA-F]{6}\b/, 'constant.other.color.hex.shapdf'],

      // Comments (after hex colors) - full line
      [/#.*$/, 'comment.line.number-sign.shapdf'],

      // Set command with parameter
      [/^\s*(set)\s+/, {
        token: 'keyword.control.shapdf',
        next: '@setParams'
      }],

      // Page command
      [/^\s*(page)\s+/, {
        token: 'keyword.control.shapdf',
        next: '@pageParams'
      }],

      // Shape commands
      [/^\s*(line|circle|rectangle)\s+/, {
        token: 'entity.name.function.shapdf',
        next: '@shapeParams'
      }],

      // Whitespace
      [/\s+/, ''],
    ],

    setParams: [
      // Must check for new patterns that would indicate we should pop first
      // Lookahead for commands or comments (but not hex colors starting with #)
      [/(?=^\s*(set|page|line|circle|rectangle)\b|^\s*#(?![0-9a-fA-F]{6}\b))/, '', '@pop'],
      [/$/, '', '@pop'],
      [/#[0-9a-fA-F]{6}\b/, 'constant.other.color.hex.shapdf'],
      [/#.*$/, 'comment.line.number-sign.shapdf', '@pop'],

      // Set-specific parameters
      [/\b(default_page_size|default_color|default_width|default_cap|default_angle|default_anchor)\b/,
        'variable.parameter.shapdf'],

      { include: '@values' },
    ],

    pageParams: [
      [/(?=^\s*(set|page|line|circle|rectangle)\b|^\s*#(?![0-9a-fA-F]{6}\b))/, '', '@pop'],
      [/$/, '', '@pop'],
      [/#[0-9a-fA-F]{6}\b/, 'constant.other.color.hex.shapdf'],
      [/#.*$/, 'comment.line.number-sign.shapdf', '@pop'],

      // Page formats
      [/\b(default|letter|legal|a4|a3|a5|tabloid)\b/, 'constant.language.shapdf'],

      { include: '@values' },
    ],

    shapeParams: [
      [/(?=^\s*(set|page|line|circle|rectangle)\b|^\s*#(?![0-9a-fA-F]{6}\b))/, '', '@pop'],
      [/$/, '', '@pop'],
      [/#[0-9a-fA-F]{6}\b/, 'constant.other.color.hex.shapdf'],
      [/#.*$/, 'comment.line.number-sign.shapdf', '@pop'],

      // Parameters (keyword=)
      [/(width|color|cap|anchor|angle)\s*=/, 'variable.parameter.shapdf'],

      // Cap types
      [/\b(round|square|butt)\b/, 'constant.language.cap.shapdf'],

      // Anchor types
      [/\b(center|north|south|east|west|northeast|northwest|southeast|southwest)\b/,
        'constant.language.anchor.shapdf'],

      { include: '@values' },
    ],

    values: [
      // Color functions
      [/(rgb|gray)\s*\(/, 'support.function.color.shapdf'],
      [/\)/, 'support.function.color.shapdf'],

      // Named colors (must check word boundary to avoid matching inside 'gray(')
      [/\b(red|green|blue|yellow|cyan|magenta|white|black|orange|purple|pink|brown)\b/,
        'constant.other.color.named.shapdf'],

      // Numbers with units
      [/\d+(?:\.\d+)?\s*(mm|cm|in|pt|deg|rad)\b/, 'constant.numeric.measurement.shapdf'],

      // Plain numbers (including decimals in parentheses like 0.2)
      [/\d+(?:\.\d+)?/, 'constant.numeric.shapdf'],

      // Commas and parentheses
      [/[(),]/, ''],

      // Whitespace
      [/\s+/, ''],
    ],
  },
};

export function registerShapdfLanguage(monaco: typeof Monaco) {
  // Register the language
  monaco.languages.register({ id: 'shapdf' });

  // Register the language configuration
  monaco.languages.setLanguageConfiguration('shapdf', shapdfLanguageConfig);

  // Register the tokens provider
  monaco.languages.setMonarchTokensProvider('shapdf', shapdfTokensProvider);

  // Define a custom theme for shapdf with proper token scopes
  monaco.editor.defineTheme('shapdf-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'comment.line.number-sign.shapdf', foreground: '6b7280', fontStyle: 'italic' },
      { token: 'keyword.control.shapdf', foreground: 'c084fc', fontStyle: 'bold' },
      { token: 'entity.name.function.shapdf', foreground: '60a5fa', fontStyle: 'bold' },
      { token: 'variable.parameter.shapdf', foreground: 'fbbf24' },
      { token: 'constant.language.shapdf', foreground: 'a78bfa' },
      { token: 'constant.language.cap.shapdf', foreground: 'a78bfa' },
      { token: 'constant.language.anchor.shapdf', foreground: 'a78bfa' },
      { token: 'constant.numeric.shapdf', foreground: 'fb923c' },
      { token: 'constant.numeric.measurement.shapdf', foreground: 'fb923c' },
      { token: 'constant.other.color.hex.shapdf', foreground: '34d399' },
      { token: 'constant.other.color.named.shapdf', foreground: '34d399' },
      { token: 'support.function.color.shapdf', foreground: '10b981' },
      { token: 'keyword.other.unit.shapdf', foreground: 'fb923c' },
    ],
    colors: {
      'editor.background': '#0f172a',
    },
  });
}
