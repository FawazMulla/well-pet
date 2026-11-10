import React, { useState } from "react";
import { Send, AlertTriangle, ShieldCheck } from "lucide-react";

interface ChatBubbleProps {
  petName: string;
}

interface Message {
  sender: "user" | "pet" | "crisis";
  text: string;
}

export const ChatBubble: React.FC<ChatBubbleProps> = ({ petName }) => {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<Message[]>([
    {
      sender: "pet",
      text: `Hi! I'm ${petName}, your wellness buddy. How are you feeling right now?`,
    },
  ]);

  const crisisRegex = /\b(suicide|kill myself|end my life|self[- ]harm|want to die|slit my|hurt myself|don't want to live)\b/i;

  const handleSend = () => {
    if (!input.trim()) return;

    const userText = input;
    const isCrisis = crisisRegex.test(userText);

    if (isCrisis) {
      setMessages((prev) => [
        ...prev,
        { sender: "user", text: userText },
        {
          sender: "crisis",
          text: "If you are feeling overwhelmed, hopeless, or having thoughts of self-harm, please know that support is available right now. You are not alone.",
        },
      ]);
    } else {
      let reply = `I'm right here beside you! Remember to take steady breaths and gentle breaks.`;
      const lower = userText.toLowerCase();
      if (lower.includes("water") || lower.includes("drink")) {
        reply = "Staying hydrated helps sustain focus! A glass of water (250ml) will refresh both of us.";
      } else if (lower.includes("eye") || lower.includes("screen")) {
        reply = "Look 20 feet away for 20 seconds to relax your eye ciliary muscles!";
      } else if (lower.includes("stretch") || lower.includes("move")) {
        reply = "Let's do a gentle shoulder roll and spine stretch together!";
      }

      setMessages((prev) => [
        ...prev,
        { sender: "user", text: userText },
        { sender: "pet", text: reply },
      ]);
    }

    setInput("");
  };

  return (
    <div className="max-w-2xl mx-auto h-[600px] flex flex-col bg-slate-900 border border-slate-800 rounded-xl overflow-hidden">
      <div className="p-4 bg-slate-850 border-b border-slate-800 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="text-xl">💬</span>
          <h3 className="font-semibold text-sm text-slate-200">Chat with {petName}</h3>
        </div>
        <div className="flex items-center gap-1 text-[11px] text-emerald-400">
          <ShieldCheck className="w-3.5 h-3.5" /> Zero Cloud Storage (RAM Only)
        </div>
      </div>

      {/* Messages Window */}
      <div className="flex-1 p-4 overflow-y-auto space-y-3">
        {messages.map((m, idx) => (
          <div
            key={idx}
            className={`flex ${m.sender === "user" ? "justify-end" : "justify-start"}`}
          >
            {m.sender === "crisis" ? (
              <div className="max-w-md p-4 bg-rose-950/80 border border-rose-600 rounded-xl text-rose-100 text-xs space-y-2">
                <div className="flex items-center gap-2 font-bold text-rose-300">
                  <AlertTriangle className="w-4 h-4" /> Immediate Support Is Available
                </div>
                <p>{m.text}</p>
                <div className="bg-rose-900/40 p-2 rounded border border-rose-800/60 font-mono text-[11px] space-y-1">
                  <div>📞 US & Canada: Call or text 988</div>
                  <div>📞 United Kingdom: Call 111</div>
                  <div>🌐 International: https://findahelpline.com</div>
                </div>
              </div>
            ) : (
              <div
                className={`max-w-xs md:max-w-md p-3 rounded-xl text-xs ${
                  m.sender === "user"
                    ? "bg-emerald-600 text-white rounded-br-none"
                    : "bg-slate-800 border border-slate-700 text-slate-200 rounded-bl-none"
                }`}
              >
                {m.text}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Input bar */}
      <div className="p-3 bg-slate-850 border-t border-slate-800 flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSend()}
          placeholder={`Talk to ${petName} (e.g. "Should I drink water?")...`}
          className="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-xs text-slate-200 focus:outline-none focus:border-emerald-500"
        />
        <button
          onClick={handleSend}
          className="p-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg"
          aria-label="Send message"
        >
          <Send className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
