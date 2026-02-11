import React, { useState } from 'react';
import { Terminal, Box, ExternalLink, Copy, Check } from 'lucide-react';

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

const Basics = ({ theme }) => {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex-1 w-full overflow-y-auto bg-[#050508] p-8 md:p-16">
      <div className="max-w-[1400px] mx-auto flex flex-col lg:flex-row gap-12">
        
        <div className="lg:w-1/3 flex flex-col gap-8">
          <div className="sticky top-0 space-y-8">
            <div>
              <div className="flex items-center gap-3 mb-6">
                <Terminal className="text-white" size={32} />
                <h2 className="text-3xl font-black text-white uppercase tracking-tighter">Installation</h2>
              </div>

              <p className="text-slate-400 font-medium mb-8 leading-relaxed">
                Install the interpreter via Cargo or grab the binaries and 
                <span className="text-white"> VS Code extension</span> from GitHub.
                More information inside Docs / Installation
              </p>

              <div className="space-y-4">
                <div className="bg-[#020205] border border-white/10 rounded-xl p-4 flex items-center justify-between group">
                  <code className="text-sm text-slate-300">cargo install sk-lang</code>
                  <button 
                    onClick={() => copyToClipboard('cargo install sk-lang')}
                    className="cursor-pointer text-slate-500 hover:text-white transition-colors"
                  >
                    {copied ? <Check size={16} className="text-green-500" /> : <Copy size={16} />}
                  </button>
                </div>

                <button 
                  onClick={() => window.open('https://github.com/aloyak/SK/releases', '_blank')}
                  className="cursor-pointer w-full flex items-center justify-center gap-3 px-6 py-4 bg-white text-black rounded-xl font-bold hover:bg-slate-200 transition-colors"
                >
                  <Box size={20} />
                  <span>Releases</span>
                  <ExternalLink size={16} />
                </button>
              </div>
            </div>

            <div className={`${theme.card} border ${theme.border} rounded-2xl p-6`}>
              <div className="flex items-center gap-2 mb-4">
                <h3 className="text-xs font-black text-white uppercase tracking-[0.2em]">Other Cool Features</h3>
              </div>
              <ul className="space-y-3">
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> Functions
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> Loops
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> Special Conditionals
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> Math, Random, Os and Time Libraries
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> REPL Interpreter
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> VS Code Extension
                </li>
                <li className="text-slate-500 text-sm font-medium flex gap-2">
                  <span className="text-white">•</span> Web IDE & Documentation
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div className="lg:w-2/3 grid grid-cols-1 md:grid-cols-2 gap-6">
          {features.map((f, i) => (
            <div 
              key={i} 
              className={`rounded-3xl p-6 flex flex-col justify-start bg-[#0D1117]`}
            >
              <h3 className="text-lg font-black text-white uppercase tracking-tight mb-2">{f.title}</h3>
              <p className="text-slate-500 text-sm font-medium mb-4 leading-tight">
                {f.desc}
              </p>
              <div className="bg-[#020205] rounded-2xl overflow-hidden">
                <img 
                  src={f.img} 
                  alt={f.title} 
                  className="w-full h-auto" 
                />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default Basics;