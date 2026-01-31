import React, { useState } from 'react';
import { Terminal, Box, ExternalLink, Copy, Check } from 'lucide-react';

const Basics = ({ theme }) => {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const features = [
    { 
      title: "Special Variables", 
      desc: "Handles known, unknown and partially known variables",
      img: "../../assets/code/vars.png"
    },
    { 
      title: "Symbolic Variables", 
      desc: "Handles formulas that may depend on unknowns or intervals",
      img: "../../assets/code/symbolics.png"
    },
    { 
      title: "Operators", 
      desc: "All arithmetic operators propagate uncertainty",
      img: "../../assets/code/operators.png"
    },
    { 
      title: "Knowledge Operators", 
      desc: "Builtin functions to work with uncertainty",
      img: "../../assets/code/knowledge_ops.png"
    },
    
  ];

  return (
    <div className="flex-1 overflow-hidden py-10 animate-in fade-in slide-in-from-bottom-4 duration-500">
      <div className="max-w-[75%] mx-auto grid grid-cols-3 gap-8">
        
        <div className={`col-span-2 ${theme.card} border-2 ${theme.border} rounded-[2rem] p-12 shadow-2xl flex flex-col`}>
          <div className="flex items-center gap-4 mb-6">
            <Terminal className="text-white" size={36} />
            <h2 className="text-4xl font-black text-white tracking-tight uppercase">How to Install</h2>
          </div>

          <p className="text-slate-400 font-medium mb-6 leading-relaxed w-[60%]">
            You can install the interpreter directly via the command line, or download the 
            pre-compiled binaries and the <span className="text-white">VS Code extension</span> from the 
            official repository.
            More information inside Docs / Installation
          </p>
          
          <div className="flex flex-col gap-6">
            <div className="bg-[#020205] rounded-2xl p-6 font-mono border border-white/5 group relative">
              <div className="flex justify-between items-center">
                <div className="flex gap-3 text-base">
                  <span className="text-[#98c379] font-bold">➜</span>
                  <span className="text-slate-600 font-bold">cargo install sk-lang</span>
                </div>
                <button 
                  onClick={() => copyToClipboard('cargo install sk-lang')}
                  className="p-2 hover:bg-white/10 rounded-lg transition-colors text-slate-500 hover:text-white"
                >
                  {copied ? <Check size={16} className="text-green-500" /> : <Copy size={16} />}
                </button>
              </div>
            </div>

            <div className="flex items-center gap-4">
              <button 
                onClick={() => window.open('https://github.com/AlmartDev/SK/releases', '_blank')}
                className="group flex items-center gap-2 px-6 py-2 bg-white text-black rounded-xl font-bold text-lg hover:bg-slate-200 transition-all duration-300"
              >
                <Box size={20} />
                <span>Releases</span>
                <ExternalLink size={18} className="opacity-0 w-0 -translate-x-2 group-hover:opacity-100 group-hover:w-5 group-hover:translate-x-0 transition-all duration-300" />
              </button>
              <span className="text-slate-500 text-sm font-bold uppercase tracking-widest">
                Interpreter & VS Code Extension
              </span>
            </div>
          </div>
        </div>

        {features.map((f, i) => (
          <div 
            key={i} 
            className={`${theme.card} border-2 ${theme.border} rounded-[2rem] p-8 shadow-xl flex flex-col`}
          >
            <div className="flex items-center gap-4 mb-4">
              <h3 className="text-xl font-black text-white uppercase tracking-tighter">{f.title}</h3>
            </div>
            
            <p className="text-slate-500 font-bold text-sm mb-6 leading-tight">
              {f.desc}
            </p>

            <div className="bg-[#020205] rounded-xl overflow-hidden border border-white/5">
              <img 
                src={f.img} 
                alt={f.title} 
                className="w-full h-auto object-cover opacity-90 hover:opacity-100 transition-opacity" 
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Basics;