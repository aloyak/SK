import React from 'react';

const Popup = ({ message, code, onClose, onCopy, copyLabel = 'Copy' }) => {
  return (
    <div className="fixed inset-0 flex items-center justify-center z-50 popup-backdrop">
      <div className="w-[92%] max-w-lg rounded-2xl border border-[#151520] bg-[#0a0a0f] p-6 shadow-2xl popup-card">
        <p className="mb-5 text-slate-400">{message}</p>
        {code && (
          <pre className="bg-[#050508] border border-[#151520] p-4 rounded-xl mb-6 overflow-x-auto text-slate-200">
            <code className="font-mono text-sm">{code}</code>
          </pre>
        )}
        <div className="flex gap-3 justify-end">
          {onCopy && (
            <button onClick={onCopy} className="px-4 py-2 rounded-lg text-slate-200 border border-[#2b2f3f] hover:text-white cursor-pointer">
              {copyLabel}
            </button>
          )}
          <button onClick={onClose} className="px-4 py-2 rounded-lg text-slate-300 border border-[#1f2230] hover:text-white cursor-pointer">
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default Popup;