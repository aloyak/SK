import { useState } from 'react';
import { Github, Box, HelpCircle, Book, Code2, ExternalLink } from 'lucide-react';

const About = ({ theme, setPage }) => {
  const openLink = (url) => {
    window.open(url, '_blank', 'noopener,noreferrer');
  };
  
  const [logoVersion, setLogo] = useState(1);

  const toggleLogo = () => {
    setLogo(logoVersion === 1 ? 2 : 1);
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center p-10 animate-in fade-in duration-500">
      <img 
        src={`../../assets/skicon${logoVersion === 2 ? '2' : ''}.png`}
        alt="SK Large Logo" 
        className="w-64 h-64 object-contain mb-12 drop-shadow-[0_0_32px_rgba(226,122,122,0.3)] cursor-pointer" 
        onClick={toggleLogo}
      />

      <div className="flex gap-3 mb-8">
        <button 
          className="cursor-pointer group flex items-center gap-2 px-4 py-1.5 bg-white text-black rounded-md font-bold text-lg hover:bg-slate-200 transition-all duration-300"
          onClick={() => openLink('https://github.com/aloyak/SK')}
        >
          <Github size={24} />
          <span>GitHub</span>
          <ExternalLink size={20} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-4 group-hover:translate-x-0 transition-all duration-300" />
        </button>
        <button 
          onClick={() => setPage('ide')}
          className="cursor-pointer group flex items-center gap-2 px-6 py-2.5 bg-[#fef08a] text-[#713f12] rounded-lg font-bold text-xl hover:opacity-90 transition-all duration-300"
        >
          <Code2 size={24} />
          <span>Web IDE</span>
          <ExternalLink size={20} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-5 group-hover:translate-x-0 transition-all duration-300" />
        </button>
      </div>

      <div className="bg-[#0a0a0f]/50 border border-white/5 rounded-xl p-8 max-w-3xl mb-8 backdrop-blur-sm">
        <p className="text-m text-slate-300 font-medium text-center leading-relaxed">
          SK is an uncertainty-based, interpreted programming language designed <br/>
          to handle unknown or partially known variables as first-class citizens. <br/>
          It features a Rust-inspired syntax and a high-performance interpreter, <br/>
          written in Rust, that supports the core specifications of the language. <br/><br/>

          Developed by <a href='https://aloyak.dev' className="underline">4loyak!</a> with &lt;3
        </p>
      </div>

      <div className="flex gap-4">
        <button 
          onClick={() => setPage('basics')}
          className="cursor-pointer group flex items-center gap-2 px-6 py-2.5 bg-[#93c5fd] text-[#1e3a8a] rounded-lg font-bold text-xl hover:opacity-90 transition-all duration-300"
        >
          <HelpCircle size={24} />
          <span>The Basics!</span>
          <ExternalLink size={20} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-5 group-hover:translate-x-0 transition-all duration-300" />
        </button>
        <button 
          onClick={() => setPage('docs')}
          className="cursor-pointer group flex items-center gap-2 px-6 py-2.5 bg-[#fbcfe8] text-[#831843] rounded-lg font-bold text-xl hover:opacity-90 transition-all duration-300"
        >
          <Book size={24} />
          <span>Docs</span>
          <ExternalLink size={20} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-5 group-hover:translate-x-0 transition-all duration-300" />
        </button>
        <button 
          onClick={() => openLink('https://crates.io/crates/sk-lang')}
          className="cursor-pointer group flex items-center gap-2 px-4 py-1.5 bg-[#f59e0b] text-[#433000] rounded-md font-bold text-lg hover:opacity-90 transition-all duration-300"
        >
          <Box size={20} />
          <span>Crates.io</span>
          <ExternalLink size={20} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-5 group-hover:translate-x-0 transition-all duration-300" />
        </button>
      </div>
    </div>
  );
};

export default About;