import Editor from '@monaco-editor/react';

const ANSI_PATTERN = /\x1b\[([0-9;]*)m/g;
const ANSI_COLORS = {
  91: '#f38ba8',
  93: '#f9e2af',
  94: '#89b4fa',
};

const parseAnsi = (text) => {
  const segments = [];
  let lastIndex = 0;
  let match;
  let style = { color: null, fontWeight: null };

  const pushSegment = (segmentText) => {
    if (!segmentText) return;
    segments.push({ text: segmentText, style: { ...style } });
  };

  while ((match = ANSI_PATTERN.exec(text)) !== null) {
    const matchIndex = match.index;
    pushSegment(text.slice(lastIndex, matchIndex));
    lastIndex = ANSI_PATTERN.lastIndex;

    const codes = match[1]
      .split(';')
      .filter(Boolean)
      .map((value) => Number(value));

    if (codes.length === 0) {
      style = { color: null, fontWeight: null };
      continue;
    }

    codes.forEach((code) => {
      if (code === 0) {
        style = { color: null, fontWeight: null };
      } else if (code === 1) {
        style = { ...style, fontWeight: '700' };
      } else if (code === 22) {
        style = { ...style, fontWeight: null };
      } else if (code === 39) {
        style = { ...style, color: null };
      } else if (ANSI_COLORS[code]) {
        style = { ...style, color: ANSI_COLORS[code] };
      }
    });
  }

  pushSegment(text.slice(lastIndex));
  return segments;
};

const IDE = ({ code, setCode, output, command, outputWidth, startResizing, handleEditorWillMount, theme }) => {
  const outputText = typeof output === 'string' ? output : String(output ?? '');
  const outputSegments = parseAnsi(outputText);

  return (
    <div className="flex-1 flex gap-2 overflow-hidden pb-6">
      <div className={`flex-1 flex flex-col ${theme.card} border-2 ${theme.border} rounded-[2rem] overflow-hidden shadow-2xl`}>
        <div className="h-24 flex items-center px-10 border-b-2 border-white/5">
          <span className="text-4xl font-black text-white tracking-tight">Input</span>
        </div>
        <div className="flex-1 select-text">
          <Editor
            height="100%"
            language="sk"
            theme="sk-theme"
            value={code}
            beforeMount={handleEditorWillMount}
            loading={<div className="h-full w-full bg-[#0a0a0f]" />}
            onChange={(value) => setCode(value)}
            options={{
              fontSize: 24,
              minimap: { enabled: false },
              lineNumbersMinChars: 4,
              fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
              scrollBeyondLastLine: false,
              automaticLayout: true,
              padding: { top: 40 },
              lineHeight: 40,
              renderLineHighlight: 'none',
              scrollbar: { vertical: 'hidden' },
            }}
          />
        </div>
      </div>

      <div 
        className="w-1 cursor-col-resize hover:bg-white/5 active:bg-white/10 self-stretch rounded-full transition-colors"
        onMouseDown={startResizing}
      />

      <div 
        className={`flex flex-col ${theme.card} border-2 ${theme.border} rounded-[2rem] overflow-hidden shadow-2xl`}
        style={{ width: `${outputWidth}px` }}
      >
        <div className="h-24 flex items-center px-10 border-b-2 border-white/5">
          <span className="text-4xl font-black text-white tracking-tight">Output</span>
        </div>
        
        <div className="flex-1 bg-[#020205] overflow-hidden select-text font-mono min-h-0">
          <div className="flex flex-col gap-4 p-10 h-full overflow-y-auto">
            <div className="flex gap-3 text-lg">
              <span className="text-[#98c379] font-bold">➜</span>
              <span className="text-slate-600 font-bold">{command}</span>
            </div>
            
            <pre className="text-2xl text-[#abb2bf] font-medium whitespace-pre-wrap leading-relaxed">
              {outputSegments.map((segment, index) => (
                <span
                  key={`${index}-${segment.text.length}`}
                  style={{
                    color: segment.style.color ?? undefined,
                    fontWeight: segment.style.fontWeight ?? undefined,
                  }}
                >
                  {segment.text}
                </span>
              ))}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
};

export default IDE;