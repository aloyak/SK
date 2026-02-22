import React, { useState, useEffect, useMemo } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Hash, ChevronRight, FileText, AlertCircle, Folder, ChevronDown } from 'lucide-react';
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
).setOptions({ gfm: true, breaks: true });

const markdownFiles = import.meta.glob('/docs/**/*.md', { query: '?raw', import: 'default' });

const Docs = ({ theme, setPage }) => {
  const location = useLocation();
  const navigate = useNavigate();
  const [content, setContent] = useState('');
  const [displayTitle, setDisplayTitle] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [expandedCategories, setExpandedCategories] = useState(new Set(['Start']));

  const formatFileName = (path) => path.split('/').pop().replace('.md', '').replace(/[-_]/g, ' ');

  const toggleCategory = (category) => {
    const newExpanded = new Set(expandedCategories);
    if (newExpanded.has(category)) {
      newExpanded.delete(category);
    } else {
      newExpanded.add(category);
    }
    setExpandedCategories(newExpanded);
  };

  const categories = useMemo(() => {
    const groups = {};
    Object.keys(markdownFiles).forEach((path) => {
      const parts = path.split('/');
      const category = parts.length > 3 ? parts[parts.length - 2] : 'General';
      (groups[category] = groups[category] || []).push(path);
    });

    return Object.fromEntries(
      Object.entries(groups).sort(([a], [b]) => 
        a === 'Start' ? -1 : b === 'Start' ? 1 : a.localeCompare(b)
      )
    );
  }, []);

  const activePath = useMemo(() => {
    const subPath = location.pathname.replace('/docs', '');
    if (!subPath || subPath === '/') {
      return Object.values(categories)[0]?.[0] || '';
    }
    return `/docs${subPath}.md`;
  }, [location.pathname, categories]);

  useEffect(() => {
    const handleLinks = (e) => {
      const link = e.target.closest('a');
      const href = link?.getAttribute('href');
      if (href?.startsWith('#page-')) {
        e.preventDefault();
        setPage?.(href.replace('#page-', ''));
      }
    };
    document.addEventListener('click', handleLinks);
    return () => document.removeEventListener('click', handleLinks);
  }, [setPage]);

  useEffect(() => {
    if (!activePath || !markdownFiles[activePath]) {
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    markdownFiles[activePath]()
      .then((text) => {
        const h1Match = text.match(/^#\s+(.*)$/m);
        setDisplayTitle(h1Match ? h1Match[1] : formatFileName(activePath));
        setContent(markedInstance.parse(h1Match ? text.replace(h1Match[0], '') : text));
      })
      .catch(() => setContent(''))
      .finally(() => setIsLoading(false));
  }, [activePath]);

  const handleFileSelect = (path) => {
    const urlPath = path.replace('/docs', '').replace('.md', '');
    navigate(`/docs${urlPath}`);
  };

  if (!theme) return null;

  return (
    <div className="flex-1 w-full overflow-y-auto scrollbar-hide bg-[#050508] py-16 text-left">
      <div className="max-w-[75%] mx-auto grid grid-cols-4 gap-12">
        <aside className="col-span-1">
          <div className="sticky top-10 flex flex-col gap-10">
            <h2 className="text-white font-black uppercase tracking-tighter text-xl">Docs</h2>
            <nav className="flex flex-col gap-8">
              {Object.entries(categories).map(([category, paths]) => (
                <div key={category} className="flex flex-col gap-2">
                  <button
                    onClick={() => toggleCategory(category)}
                    className="cursor-pointer flex items-center gap-2 px-2 mb-1 opacity-40 hover:opacity-60 transition-opacity"
                  >
                    <ChevronDown 
                      size={14} 
                      className={`text-white transition-transform ${expandedCategories.has(category) ? 'rotate-0' : '-rotate-90'}`}
                    />
                    <Folder size={12} className="text-white" />
                    <span className="text-[10px] font-black uppercase tracking-[0.2em] text-white">{category}</span>
                  </button>
                  {expandedCategories.has(category) && (
                    <div className="flex flex-col gap-2">
                      {paths.map((path) => (
                        <button
                          key={path}
                          onClick={() => handleFileSelect(path)}
                          className={`cursor-pointer flex items-center gap-3 px-4 py-2.5 rounded-xl transition-all group ${
                            activePath === path ? 'bg-white text-black font-bold shadow-lg scale-[1.02]' : 'text-slate-400 hover:text-white hover:bg-white/5'
                          }`}
                        >
                          <Hash size={14} className={activePath === path ? 'text-black' : 'text-slate-600'} />
                          <span className="text-sm capitalize">{formatFileName(path)}</span>
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              ))}
            </nav>
          </div>
        </aside>

        <main className="col-span-3">
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
                <span className="text-white text-xs font-bold uppercase tracking-widest">{formatFileName(activePath)}</span>
              </div>
              <h1 className="text-6xl font-black text-white mb-10 uppercase tracking-tighter leading-none">{displayTitle}</h1>
              <div className={`p-12 rounded-[2rem] border-2 ${theme.border} ${theme.card} bg-opacity-50 shadow-2xl overflow-hidden`}>
                <article className="prose-custom select-text max-w-none">
                  <div 
                    className="markdown-content"
                    dangerouslySetInnerHTML={{ __html: content }} 
                  />
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