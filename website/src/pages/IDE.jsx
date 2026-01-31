import Editor from '@monaco-editor/react';

const IDE = ({ code, setCode, output, outputWidth, startResizing, handleEditorWillMount, theme }) => {
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
              scrollbar: { vertical: 'hidden', horizontal: 'hidden' },
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
        
        <div className="flex-1 bg-[#020205] overflow-hidden select-text font-mono">
          <div className="flex flex-col gap-4 p-10">
            <div className="flex gap-3 text-lg">
              <span className="text-[#98c379] font-bold">➜</span>
              <span className="text-slate-600 font-bold">sk file.sk</span>
            </div>
            
            <pre className="text-3xl text-[#abb2bf] font-medium whitespace-pre-wrap leading-relaxed">
              {output}
            </pre>
            
            <div className="flex gap-2 animate-pulse">
              <span className="text-white font-bold text-3xl">█</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default IDE;