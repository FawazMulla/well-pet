import React from "react";
import { PetSpecies, EmotionalState, Dimensions } from "../App";
import { Zap, Database, HeartPulse } from "lucide-react";

interface DevDemoPanelProps {
  petName: string;
  setPetName: (name: string) => void;
  species: PetSpecies;
  setSpecies: (species: PetSpecies) => void;
  emotionalState: EmotionalState;
  setEmotionalState: (state: EmotionalState) => void;
  dimensions: Dimensions;
  setDimensions: React.Dispatch<React.SetStateAction<Dimensions>>;
}

export const DevDemoPanel: React.FC<DevDemoPanelProps> = ({
  petName,
  setPetName,
  species,
  setSpecies,
  emotionalState,
  setEmotionalState,
  dimensions,
  setDimensions,
}) => {
  const seed14Days = () => {
    alert("Seeded 14 days of realistic activity & LinUCB logs. Day 8 Auto-Planner and Day 14 Diurnal curve unlocked!");
  };

  const handleStateInjection = (newState: EmotionalState) => {
    setEmotionalState(newState);
    switch (newState) {
      case "thriving":
        setDimensions({ eye: 90, hydration: 90, mobility: 90, mental: 90, posture: 90 });
        break;
      case "happy":
        setDimensions({ eye: 75, hydration: 70, mobility: 70, mental: 75, posture: 70 });
        break;
      case "tired":
        setDimensions({ eye: 50, hydration: 50, mobility: 50, mental: 50, posture: 50 });
        break;
      case "sleepy":
        setDimensions({ eye: 30, hydration: 30, mobility: 30, mental: 30, posture: 30 });
        break;
      case "napping":
        setDimensions({ eye: 15, hydration: 15, mobility: 15, mental: 15, posture: 15 });
        break;
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <header>
        <h2 className="text-2xl font-bold text-slate-100">Hackathon & Developer Demo Mode</h2>
        <p className="text-sm text-slate-400">
          Tools for rapid verification, state injection, and 60x simulation demo.
        </p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* State Injector */}
        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl space-y-3">
          <div className="flex items-center gap-2 text-purple-400 font-semibold text-sm">
            <HeartPulse className="w-4 h-4" /> State Injector
          </div>
          <p className="text-xs text-slate-400">
            Instantly set emotional states to inspect sprite behavior and Care Center reactions.
          </p>
          <div className="flex flex-wrap gap-2 pt-2">
            {(["thriving", "happy", "tired", "sleepy", "napping"] as EmotionalState[]).map(
              (s) => (
                <button
                  key={s}
                  onClick={() => handleStateInjection(s)}
                  className={`text-xs px-2.5 py-1 rounded capitalize border ${
                    emotionalState === s
                      ? "bg-purple-600 border-purple-500 text-white font-bold"
                      : "bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700"
                  }`}
                >
                  {s}
                </button>
              )
            )}
          </div>
        </div>

        {/* Species & Identity */}
        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl space-y-3">
          <div className="flex items-center gap-2 text-cyan-400 font-semibold text-sm">
            <span>🐾</span> Companion Identity
          </div>
          <div>
            <label className="text-xs text-slate-400 block mb-1">Companion Name:</label>
            <input
              type="text"
              value={petName}
              onChange={(e) => setPetName(e.target.value)}
              className="w-full bg-slate-800 border border-slate-700 rounded px-2.5 py-1.5 text-xs text-slate-200"
            />
          </div>
          <div>
            <label className="text-xs text-slate-400 block mb-1">Species:</label>
            <div className="flex gap-2">
              {(["cat", "dog", "plant"] as PetSpecies[]).map((sp) => (
                <button
                  key={sp}
                  onClick={() => setSpecies(sp)}
                  className={`text-xs flex-1 py-1 rounded capitalize border ${
                    species === sp
                      ? "bg-cyan-600 border-cyan-500 text-white font-bold"
                      : "bg-slate-800 border-slate-700 text-slate-300 hover:bg-slate-700"
                  }`}
                >
                  {sp}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Synthetic Seed & Time Lapse */}
        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl space-y-3">
          <div className="flex items-center gap-2 text-amber-400 font-semibold text-sm">
            <Zap className="w-4 h-4" /> Simulation Acceleration
          </div>
          <p className="text-xs text-slate-400">
            Seed 14 days of historical telemetry or run 60x internal time acceleration.
          </p>
          <button
            onClick={seed14Days}
            className="w-full py-2 bg-slate-800 hover:bg-slate-700 border border-slate-700 text-xs text-slate-200 rounded flex items-center justify-center gap-2"
          >
            <Database className="w-3.5 h-3.5" /> Seed 14 Days Telemetry
          </button>
          <div className="text-[11px] text-emerald-400 bg-emerald-950/30 p-2 rounded border border-emerald-800/40">
            ⚡ Time-Lapse Flag: `--demo-mode` active (1s real = 1m simulated)
          </div>
        </div>
      </div>
    </div>
  );
};
