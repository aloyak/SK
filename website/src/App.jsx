import React, { useState, useRef, useEffect, useMemo } from 'react';
import { BrowserRouter, Routes, Route, Navigate, useNavigate, useLocation } from 'react-router-dom';
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

function AppRoutes({ theme }) {
  const navigate = useNavigate();
  const location = useLocation();

  const [code, setCode] = useState(
    '// The SK Programming Language\n' + 
    '\n' +
    'print("Hello, World!")\n' +
    '\n' +
    'fn fibonacci(n, previous = 0, current = 1) {\n' +
    '    if n > 0 {\n' +
    '        print(current)\n' +
    '        fibonacci(n - 1, current, previous + current)\n' +
    '    }\n' + 
    '}\n' +
    '\n' +
    'fibonacci(10)\n' +
    '\n' +
    'let variable = [0..10] // partially known variable\n' +
    'print("Rate this language from " + str(variable) + "!")\n' +
    '\n' +
    '// Find many more examples to try at: \n' +
    '// https://github.com/aloyak/SK/tree/main/interpreter/examples\n'
  );
  const [output, setOutput] = useState('Run the code to see the output');
  const [outputWidth, setOutputWidth] = useState(900);
  const [isInitialized, setIsInitialized] = useState(false);
  const [command, setCommand] = useState('SK --version');
  
  const isResizing = useRef(false);
  const fileInputRef = useRef(null);
  const t = theme;

  const currentPage = useMemo(() => {
    return location.pathname.replace('/', '') || 'about';
  }, [location.pathname]);

  const setPage = (nextPage) => {
    const target = `/${nextPage}`;
    if (location.pathname !== target) navigate(target);
  };

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
    if (currentPage === 'ide') {
      setCommand('SK --version');
      if (!isInitialized) setOutput('Loading interpreter...');
    }
  }, [currentPage, isInitialized]);

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
          [/\b(symbolic|let|unknown|quiet|fn|return|if|elif|else|import|as|pub|loop|break|continue)\b/, 'keyword'],
          [/\b(certain|possible|impossible|known)\b/, 'builtins'],
          [/panic!/, 'builtins'],
          [/\b(print|input|str|num|kind|resolve)\b/, 'builtins'],
          [/\b(true|false|partial)\b/, 'booleans'],
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
        { token: 'builtins', foreground: '89b4fa' },
        { token: 'booleans', foreground: 'b5c9e8' },
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
    setCommand('SK --safe main.sk');
    setOutput('Running...');

    const inputMatches = code.match(/input\(.*?\)/g) || [];
    const userInputs = [];

    if (inputMatches.length > 0) {
      for (let i = 0; i < inputMatches.length; i++) {
        const val = window.prompt(`Input required (${i + 1}/${inputMatches.length}):\n${inputMatches[i]}`);
        if (val === null) {
          setOutput("Execution cancelled by user.");
          return;
        }
        userInputs.push(val);
      }
    }

    try {
      const res = await fetch('/api/eval', { 
        method: 'POST', 
        body: JSON.stringify({ code, inputs: userInputs }) 
      });
      setOutput(await res.text());
    } catch (err) {
      setOutput("Failed to connect to runner.");
    }
  };

  return (
    <div className={`h-screen flex flex-col ${t.bg} font-sans p-10 select-none`}>
      <input type="file" ref={fileInputRef} onChange={handleUpload} className="hidden" accept=".sk" />
      <Header currentPage={currentPage} onRun={handleRun} onDownload={handleDownload} onUpload={handleUpload} setPage={setPage} theme={t} />
      <Routes>
        <Route path="/about" element={<About theme={t} setPage={setPage} />} />
        <Route path="/ide" element={<IDE {...{code, setCode, output, command, outputWidth, startResizing, handleEditorWillMount, theme: t}} />} />
        <Route path="/docs" element={<Docs theme={t} setPage={setPage} />} />
        <Route path="/basics" element={<Basics theme={t} />} />
        <Route path="/" element={<Navigate to="/about" replace />} />
        <Route path="*" element={<Navigate to="/about" replace />} />
      </Routes>
    </div>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AppRoutes theme={THEMES.sk} />
    </BrowserRouter>
  );
}

export default App;