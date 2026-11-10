import React, { useState } from "react";
import { PetOverlay } from "./components/PetOverlay";
import { CareCenter } from "./components/CareCenter";
import { DayPlanner } from "./components/DayPlanner";
import { Dashboard } from "./components/Dashboard";
import { ChatBubble } from "./components/ChatBubble";
import { DevDemoPanel } from "./components/DevDemoPanel";
import { Heart, Calendar, BarChart2, MessageSquare, Shield, Sliders } from "lucide-react";

export type EmotionalState = "thriving" | "happy" | "tired" | "sleepy" | "napping";
export type PetSpecies = "cat" | "dog" | "plant";

export interface Dimensions {
  eye: number;
  hydration: number;
  mobility: number;
  mental: number;
  posture: number;
}

export default function App() {
  const [petName, setPetName] = useState("Luna");
  const [species, setSpecies] = useState<PetSpecies>("cat");
  const [activeTab, setActiveTab] = useState<"care" | "planner" | "dashboard" | "chat" | "demo">("care");
  const [focusMode, setFocusMode] = useState(false);
  const [level, setLevel] = useState(2);
  const [xp, setXp] = useState(120);
  const [activeHabitDays, setActiveHabitDays] = useState(3);
  const [emotionalState, setEmotionalState] = useState<EmotionalState>("thriving");
  const [dimensions, setDimensions] = useState<Dimensions>({
    eye: 85,
    hydration: 80,
    mobility: 90,
    mental: 85,
    posture: 90,
  });

  const compositeScore = Math.round(
    0.25 * dimensions.mobility +
    0.25 * dimensions.mental +
    0.20 * dimensions.eye +
    0.15 * dimensions.hydration +
    0.15 * dimensions.posture
  );

  const awardXp = (amount: number) => {
    setXp((prev) => prev + amount);
  };

  return (
    <div className="flex h-screen bg-slate-950 text-slate-100 overflow-hidden">
      {/* Sidebar Nav */}
      <nav className="w-64 border-r border-slate-800 bg-slate-900/70 p-4 flex flex-col justify-between" aria-label="Main Navigation">
        <div>
          <div className="flex items-center gap-2 mb-6 px-2">
            <span className="text-2xl">🐾</span>
            <div>
              <h1 className="font-bold text-lg text-emerald-400">WellPet v1.2</h1>
              <p className="text-xs text-slate-400">Adaptive Wellness Companion</p>
            </div>
          </div>

          <div className="space-y-1">
            <button
              onClick={() => setActiveTab("care")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                activeTab === "care" ? "bg-emerald-600/30 text-emerald-400 border border-emerald-500/30" : "text-slate-300 hover:bg-slate-800"
              }`}
            >
              <Heart className="w-4 h-4" /> Care Center
            </button>
            <button
              onClick={() => setActiveTab("planner")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                activeTab === "planner" ? "bg-emerald-600/30 text-emerald-400 border border-emerald-500/30" : "text-slate-300 hover:bg-slate-800"
              }`}
            >
              <Calendar className="w-4 h-4" /> Day Planner
            </button>
            <button
              onClick={() => setActiveTab("dashboard")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                activeTab === "dashboard" ? "bg-emerald-600/30 text-emerald-400 border border-emerald-500/30" : "text-slate-300 hover:bg-slate-800"
              }`}
            >
              <BarChart2 className="w-4 h-4" /> Analytics
            </button>
            <button
              onClick={() => setActiveTab("chat")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                activeTab === "chat" ? "bg-emerald-600/30 text-emerald-400 border border-emerald-500/30" : "text-slate-300 hover:bg-slate-800"
              }`}
            >
              <MessageSquare className="w-4 h-4" /> Chat Companion
            </button>
            <button
              onClick={() => setActiveTab("demo")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition ${
                activeTab === "demo" ? "bg-purple-600/30 text-purple-400 border border-purple-500/30" : "text-slate-300 hover:bg-slate-800"
              }`}
            >
              <Sliders className="w-4 h-4" /> Demo & DevTools
            </button>
          </div>
        </div>

        {/* Pet Summary Card */}
        <div className="bg-slate-900 border border-slate-800 p-3 rounded-xl">
          <div className="flex items-center justify-between text-xs text-slate-400 mb-2">
            <span>Level {level}</span>
            <span>{xp} XP</span>
          </div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full overflow-hidden mb-3">
            <div className="bg-emerald-400 h-full" style={{ width: `${Math.min(100, (xp / 2402) * 100)}%` }}></div>
          </div>
          <div className="flex items-center justify-between text-xs">
            <span className="capitalize font-medium text-slate-300">{petName} ({species})</span>
            <span className="text-emerald-400 font-semibold">{compositeScore}%</span>
          </div>
          <button
            onClick={() => setFocusMode(!focusMode)}
            className="mt-3 w-full text-xs py-1.5 px-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700 flex items-center justify-center gap-1.5"
          >
            <Shield className="w-3 h-3" />
            {focusMode ? "Disable Focus Mode" : "Focus Mode (Tray)"}
          </button>
        </div>
      </nav>

      {/* Main View Area */}
      <main className="flex-1 p-6 overflow-y-auto relative">
        {activeTab === "care" && (
          <CareCenter
            dimensions={dimensions}
            setDimensions={setDimensions}
            onCompleteActivity={(tier, xpAward) => awardXp(xpAward)}
          />
        )}
        {activeTab === "planner" && <DayPlanner />}
        {activeTab === "dashboard" && <Dashboard compositeScore={compositeScore} />}
        {activeTab === "chat" && <ChatBubble petName={petName} />}
        {activeTab === "demo" && (
          <DevDemoPanel
            petName={petName}
            setPetName={setPetName}
            species={species}
            setSpecies={setSpecies}
            emotionalState={emotionalState}
            setEmotionalState={setEmotionalState}
            dimensions={dimensions}
            setDimensions={setDimensions}
          />
        )}

        {/* Floating Desktop Companion Overlay */}
        {!focusMode && (
          <PetOverlay
            name={petName}
            species={species}
            state={emotionalState}
            compositeScore={compositeScore}
          />
        )}
      </main>
    </div>
  );
}
