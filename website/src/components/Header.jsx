import React from 'react';
import { Play, Download, Upload } from 'lucide-react';

const Header = ({ currentPage, onRun, onDownload, onUpload, setPage, theme }) => {
  const getNavLinkClass = (pageName) => {
    const base = "transition hover:text-white cursor-pointer";
    const active = "text-white";
    const inactive = "text-slate-500";
    return `${base} ${currentPage === pageName ? active : inactive}`;
  };

  return (
    <header className="h-10 grid grid-cols-3 items-center px-1 mb-8">
      <div className="flex items-center gap-4 cursor-pointer" onClick={() => setPage('about')}>
        <img src="../../assets/skicon2.png" alt="SK Logo" className="w-12 h-12 object-contain" />
        {currentPage === 'ide' && (
          <span className="text-sm text-slate-800 mt-2 font-black tracking-[0.3em] animate-in fade-in duration-300">
            IDE v1.0.2
          </span>
        )}
      </div>
      
      <nav className="flex justify-center gap-12 text-xl font-black uppercase tracking-tighter">
        <button onClick={() => setPage('ide')} className={getNavLinkClass('ide')}>IDE</button>
        <button onClick={() => setPage('docs')} className={getNavLinkClass('docs')}>Docs</button>
        <button onClick={() => setPage('basics')} className={getNavLinkClass('basics')}>Basics</button>
        <button onClick={() => setPage('about')} className={getNavLinkClass('about')}>About</button>
      </nav>

      <div className="flex justify-end items-center gap-6 text-slate-600">
        {currentPage === 'ide' && (
          <div className="flex items-center gap-6 animate-in slide-in-from-right-4 duration-300">
            <button onClick={onRun} className={`${theme.buttonHover} cursor-pointer`}>
              <Play size={32} strokeWidth={2} />
            </button>
            <button onClick={onDownload} className={`${theme.buttonHover} cursor-pointer`}>
              <Download size={32} strokeWidth={2} />
            </button>
            <label className={`cursor-pointer ${theme.buttonHover} cursor-pointer`}>
              <input type="file" className="hidden" onChange={onUpload} />
              <Upload size={32} strokeWidth={2} />
            </label>
          </div>
        )}
      </div>
    </header>
  );
};

export default Header;