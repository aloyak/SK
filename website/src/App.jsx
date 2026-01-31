import React, { useState, useRef } from 'react';
import Header from './components/Header';
import IDE from './pages/IDE';
import About from './pages/About';
import Docs from './pages/Docs';
import Basics from './pages/Basics';

const THEMES = {
  sk: {
    bg: 'bg-[#050508]',
    card: 'bg-[#0a0a0f]',
    border: 'border-[#151520]',
    text: 'text-slate-400',
    title: 'text-[#e27a7a]',
    buttonHover: 'hover:text-white',
  }
};

function App() {
  const [page, setPage] = useState('about');
  const [code, setCode] = useState('// The SK Programming Language \nprint("Hello, World!")');
  const [output, setOutput] = useState('Run the code to see the output');
  const [outputWidth, setOutputWidth] = useState(700);
  const isResizing = useRef(false);

  const t = THEMES.sk;

  const handleEditorWillMount = (monaco) => {
    monaco.languages.register({ id: 'sk' });
    monaco.languages.setMonarchTokensProvider('sk', {
      tokenizer: {
        root: [
          [/\b(fn|return|if|else|while|for)\b/, 'keyword'],
          [/\b(print)\b/, 'custom-print'],
          [/\/\/.*$/, 'comment'],
          [/"[^"]*"/, 'string'],
          [/\d+/, 'number'],
        ]
      }
    });

    monaco.editor.defineTheme('sk-theme', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'keyword', foreground: 'c678dd' },
        { token: 'custom-print', foreground: '98c379' },
        { token: 'comment', foreground: '5c6370' },
        { token: 'string', foreground: 'd19a66' },
      ],
      colors: {
        'editor.background': '#0a0a0f',
        'editor.lineHighlightBackground': '#00000000',
        'editorCursor.foreground': '#ffffff',
      }
    });
  };

  const startResizing = () => {
    isResizing.current = true;
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', stopResizing);
    document.body.style.cursor = 'col-resize';
  };

  const stopResizing = () => {
    isResizing.current = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', stopResizing);
    document.body.style.cursor = 'default';
  };

  const handleMouseMove = (e) => {
    if (!isResizing.current) return;
    const newWidth = window.innerWidth - e.clientX;
    if (newWidth > 150 && newWidth < window.innerWidth - 300) {
      setOutputWidth(newWidth);
    }
  };

  const handleDownload = () => {
    const element = document.createElement("a");
    const file = new Blob([code], {type: 'text/plain'});
    element.href = URL.createObjectURL(file);
    element.download = "main.sk";
    document.body.appendChild(element);
    element.click();
  };

  const handleUpload = (e) => {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => setCode(e.target.result);
      reader.readAsText(file);
    }
  };

  return (
    <div className={`h-screen flex flex-col ${t.bg} font-sans p-10 select-none`}>
      <Header 
        currentPage={page}
        onRun={() => setOutput('Hello, World!')}
        onDownload={handleDownload}
        onUpload={handleUpload}
        setPage={setPage}
        theme={t}
      />

      {page === 'ide' && (
        <IDE 
          code={code}
          setCode={setCode}
          output={output}
          outputWidth={outputWidth}
          startResizing={startResizing}
          handleEditorWillMount={handleEditorWillMount}
          theme={t}
        />
      )}

      {page === 'about' && <About theme={t} setPage={setPage} />}
      {page === 'docs' && <Docs theme={t} setPage={setPage} />}
      {page === 'basics' && <Basics theme={t} setPage={setPage} />}
    </div>
  );
}

export default App;