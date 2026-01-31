import React, { useState, useEffect, useMemo } from 'react';
import { Hash, ChevronRight, FileText, AlertCircle, Terminal, Folder } from 'lucide-react';
import { Marked } from 'marked';
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';

const markedInstance = new Marked(
  markedHighlight({
    langPrefix: 'hljs language-',
    highlight(code, lang) {
      const language = hljs.getLanguage(lang) ? lang : 'plaintext';
      return hljs.highlight(code, { language }).value;
    }
  })
);

markedInstance.setOptions({
  gfm: true,
  breaks: true,
});

const markdownFiles = import.meta.glob('/docs/**/*.md', { query: '?raw', import: 'default' });

const Docs = ({ theme, setPage }) => {
  const [activePath, setActivePath] = useState('');
  const [content, setContent] = useState('');
  const [displayTitle, setDisplayTitle] = useState('');
  const [isLoading, setIsLoading] = useState(true);

  const categories = useMemo(() => {
    const groups = {};
    Object.keys(markdownFiles).forEach((path) => {
      const parts = path.split('/');
      const category = parts.length > 3 ? parts[parts.length - 2] : 'General';
      if (!groups[category]) groups[category] = [];
      groups[category].push(path);
    });

    return Object.keys(groups)
      .sort((a, b) => {
        if (a === 'Start') return -1;
        if (b === 'Start') return 1;
        return a.localeCompare(b);
      })
      .reduce((acc, key) => {
        acc[key] = groups[key];
        return acc;
      }, {});
  }, []);

  useEffect(() => {
    const handleInternalLinks = (e) => {
      const link = e.target.closest('a');
      if (link && link.getAttribute('href')?.startsWith('#page-')) {
        e.preventDefault();
        const targetPage = link.getAttribute('href').replace('#page-', '');
        if (setPage) setPage(targetPage);
      }
    };

    document.addEventListener('click', handleInternalLinks);
    return () => document.removeEventListener('click', handleInternalLinks);
  }, [setPage]);

  useEffect(() => {
    if (!activePath) {
      const categoryKeys = Object.keys(categories);
      if (categoryKeys.length > 0) {
        const firstCategory = categoryKeys[0];
        const firstFile = categories[firstCategory][0];
        setActivePath(firstFile);
      }
    }
  }, [categories, activePath]);

  useEffect(() => {
    if (!activePath || !markdownFiles[activePath]) return;

    setIsLoading(true);
    markdownFiles[activePath]()
      .then((text) => {
        const h1Match = text.match(/^#\s+(.*)$/m);
        const extractedTitle = h1Match ? h1Match[1] : formatFileName(activePath);
        const bodyWithoutTitle = h1Match ? text.replace(h1Match[0], '') : text;

        setDisplayTitle(extractedTitle);
        const htmlContent = markedInstance.parse(bodyWithoutTitle);
        setContent(htmlContent);
        setIsLoading(false);
      })
      .catch((err) => {
        console.error("Markdown Error:", err);
        setContent('');
        setIsLoading(false);
      });
  }, [activePath]);

  const formatFileName = (path) => {
    return path.split('/').pop().replace('.md', '').replace(/[-_]/g, ' ');
  };

  if (!theme) return null;

  return (
    <div className="flex-1 w-full overflow-y-auto scrollbar-hide bg-[#050508] py-16">
      <div className="max-w-[75%] mx-auto grid grid-cols-4 gap-12">
        <aside className="col-span-1">
          <div className="sticky top-10 flex flex-col gap-10">
            <div className="flex items-center gap-3">
              <h2 className="text-white font-black uppercase tracking-tighter text-xl text-left">Docs</h2>
            </div>
            
            <nav className="flex flex-col gap-8 text-left">
              {Object.entries(categories).map(([category, paths]) => (
                <div key={category} className="flex flex-col gap-2">
                  <div className="flex items-center gap-2 px-2 mb-1 opacity-40">
                    <Folder size={12} className="text-white" />
                    <span className="text-[10px] font-black uppercase tracking-[0.2em] text-white">
                      {category}
                    </span>
                  </div>
                  <div className="flex flex-col gap-1">
                    {paths.map((path) => (
                      <button
                        key={path}
                        onClick={() => setActivePath(path)}
                        className={`flex items-center gap-3 w-full text-left px-4 py-2.5 rounded-xl transition-all duration-200 group ${
                          activePath === path 
                            ? 'bg-white text-black font-bold shadow-lg scale-[1.02]' 
                            : 'text-slate-400 hover:text-white hover:bg-white/5'
                        }`}
                      >
                        <Hash size={14} className={activePath === path ? 'text-black' : 'text-slate-600'} />
                        <span className="text-sm capitalize">{formatFileName(path)}</span>
                      </button>
                    ))}
                  </div>
                </div>
              ))}
            </nav>
          </div>
        </aside>

        <main className="col-span-3 text-left">
          {isLoading ? (
            <div className="animate-pulse space-y-8">
              <div className="h-14 w-1/2 bg-white/5 rounded-2xl" />
              <div className="h-96 w-full bg-white/5 rounded-[2rem]" />
            </div>
          ) : content ? (
            <div className="animate-in fade-in slide-in-from-right-4 duration-500">
              <div className="flex items-center gap-3 mb-8 opacity-40">
                <FileText size={14} className="text-white" />
                <span className="text-white text-xs font-bold uppercase tracking-widest">Docs</span>
                <ChevronRight size={12} className="text-white" />
                <span className="text-white text-xs font-bold uppercase tracking-widest">
                  {formatFileName(activePath)}
                </span>
              </div>

              <h1 className="text-6xl font-black text-white mb-10 uppercase tracking-tighter leading-none">
                {displayTitle}
              </h1>

              <div className={`p-12 rounded-[2rem] border-2 ${theme.border} ${theme.card} bg-opacity-50 shadow-2xl overflow-hidden`}>
                <article className="prose-custom select-text">
                  <div dangerouslySetInnerHTML={{ __html: content }} />
                </article>
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center py-24 opacity-20">
              <AlertCircle size={64} className="text-white mb-4" />
              <p className="text-white font-bold uppercase tracking-widest text-lg">Empty File</p>
            </div>
          )}
        </main>
      </div>
    </div>
  );
};

export default Docs;