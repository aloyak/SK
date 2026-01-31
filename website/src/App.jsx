import React, { useState, useRef, useEffect } from 'react';
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
  const [isInitialized, setIsInitialized] = useState(false);
  const [command, setCommand] = useState('SK --version');
  
  const isResizing = useRef(false);
  const fileInputRef = useRef(null);
  const t = THEMES.sk;

  useEffect(() => {
    (async () => {
      try {
        const res = await fetch('/api/eval');
        setOutput(await res.text());
        setIsInitialized(true);
      } catch (err) {
        console.error("Failed to init:", err);
      }
    })();
  }, []);

  useEffect(() => {
    if (page === 'ide') {
      setCommand('SK --version');
      if (!isInitialized) setOutput('Loading interpreter...');
    }
  }, [page, isInitialized]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (!(e.ctrlKey || e.metaKey)) return;
      if (e.key === 's') { e.preventDefault(); handleDownload(); }
      if (e.key === 'o') { e.preventDefault(); fileInputRef.current?.click(); }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [code]);

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
        { token: 'keyword', foreground: 'cba6f7', fontStyle: 'bold' },
        { token: 'custom-print', foreground: '89b4fa' },
        { token: 'comment', foreground: '6c7086', fontStyle: 'italic' },
        { token: 'string', foreground: 'a6e3a1' },
        { token: 'number', foreground: 'fab387' },
      ],
      colors: { 'editor.background': '#0a0a0f', 'editor.lineHighlightBackground': '#1e1e2e10' }
    });
  };

  const startResizing = () => {
    isResizing.current = true;
    const onMove = (e) => {
      if (!isResizing.current) return;
      const newWidth = window.innerWidth - e.clientX;
      if (newWidth > 150 && newWidth < window.innerWidth - 300) setOutputWidth(newWidth);
    };
    const onUp = () => {
      isResizing.current = false;
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    };
    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  };

  const handleDownload = () => {
    const blob = new Blob([code], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const link = Object.assign(document.createElement("a"), { href: url, download: "main.sk" });
    link.click();
    URL.revokeObjectURL(url);
  };

  const handleUpload = (e) => {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => setCode(ev.target.result);
    reader.readAsText(file);
    e.target.value = null;
  };

  const handleRun = async () => {
    setCommand('SK main.sk');
    setOutput('Running...');
    try {
      const res = await fetch('/api/eval', { method: 'POST', body: code });
      setOutput(await res.text());
    } catch (err) {
      setOutput("Failed to connect to runner.");
    }
  };

  const PAGES = {
    ide: <IDE {...{code, setCode, output, command, outputWidth, startResizing, handleEditorWillMount, theme: t}} />,
    about: <About theme={t} setPage={setPage} />,
    docs: <Docs theme={t} setPage={setPage} />,
    basics: <Basics theme={t} />
  };

  return (
    <div className={`h-screen flex flex-col ${t.bg} font-sans p-10 select-none`}>
      <input type="file" ref={fileInputRef} onChange={handleUpload} className="hidden" accept=".sk" />
      <Header currentPage={page} onRun={handleRun} onDownload={handleDownload} onUpload={() => fileInputRef.current?.click()} setPage={setPage} theme={t} />
      {PAGES[page]}
    </div>
  );
}

export default App;