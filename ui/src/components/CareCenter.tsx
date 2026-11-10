import React, { useState } from "react";
import { Dimensions } from "../App";
import { Eye, Droplets, Activity, Brain, UserCheck, CheckCircle } from "lucide-react";

interface CareCenterProps {
  dimensions: Dimensions;
  setDimensions: React.Dispatch<React.SetStateAction<Dimensions>>;
  onCompleteActivity: (tier: string, xp: number) => void;
}

export const CareCenter: React.FC<CareCenterProps> = ({
  dimensions,
  setDimensions,
  onCompleteActivity,
}) => {
  const [activeExercise, setActiveExercise] = useState<string | null>(null);
  const [exerciseTimer, setExerciseTimer] = useState(0);

  const startExercise = (name: string, durationSec: number) => {
    setActiveExercise(name);
    setExerciseTimer(durationSec);
  };

  const completeIntervention = (type: "eye" | "water" | "stretch" | "breathe" | "posture") => {
    setDimensions((prev) => {
      switch (type) {
        case "eye":
          return { ...prev, eye: Math.min(100, prev.eye + 15) };
        case "water":
          return { ...prev, hydration: Math.min(100, prev.hydration + 25) };
        case "stretch":
          return { ...prev, mobility: Math.min(100, prev.mobility + 25), posture: Math.min(100, prev.posture + 20) };
        case "breathe":
          return { ...prev, mental: Math.min(100, prev.mental + 25) };
        case "posture":
          return { ...prev, posture: Math.min(100, prev.posture + 20) };
      }
    });
    onCompleteActivity("TierCGuided", 12);
    setActiveExercise(null);
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <header>
        <h2 className="text-2xl font-bold text-slate-100">Care Center</h2>
        <p className="text-sm text-slate-400">
          Collaborative wellness micro-interventions and comfort indicators.
        </p>
      </header>

      {/* 5 Dimension Cards */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div className="bg-slate-900 border border-slate-800 p-4 rounded-xl">
          <div className="flex items-center gap-2 text-cyan-400 mb-2">
            <Eye className="w-5 h-5" />
            <h3 className="font-semibold text-xs">Eye Comfort</h3>
          </div>
          <div className="text-2xl font-bold text-slate-100">{dimensions.eye}%</div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full mt-2">
            <div className="bg-cyan-400 h-full rounded-full" style={{ width: `${dimensions.eye}%` }}></div>
          </div>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-4 rounded-xl">
          <div className="flex items-center gap-2 text-blue-400 mb-2">
            <Droplets className="w-5 h-5" />
            <h3 className="font-semibold text-xs">Hydration</h3>
          </div>
          <div className="text-2xl font-bold text-slate-100">{dimensions.hydration}%</div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full mt-2">
            <div className="bg-blue-400 h-full rounded-full" style={{ width: `${dimensions.hydration}%` }}></div>
          </div>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-4 rounded-xl">
          <div className="flex items-center gap-2 text-emerald-400 mb-2">
            <Activity className="w-5 h-5" />
            <h3 className="font-semibold text-xs">Mobility</h3>
          </div>
          <div className="text-2xl font-bold text-slate-100">{dimensions.mobility}%</div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full mt-2">
            <div className="bg-emerald-400 h-full rounded-full" style={{ width: `${dimensions.mobility}%` }}></div>
          </div>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-4 rounded-xl">
          <div className="flex items-center gap-2 text-purple-400 mb-2">
            <Brain className="w-5 h-5" />
            <h3 className="font-semibold text-xs">Mental Freshness</h3>
          </div>
          <div className="text-2xl font-bold text-slate-100">{dimensions.mental}%</div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full mt-2">
            <div className="bg-purple-400 h-full rounded-full" style={{ width: `${dimensions.mental}%` }}></div>
          </div>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-4 rounded-xl">
          <div className="flex items-center gap-2 text-amber-400 mb-2">
            <UserCheck className="w-5 h-5" />
            <h3 className="font-semibold text-xs">Posture Comfort</h3>
          </div>
          <div className="text-2xl font-bold text-slate-100">{dimensions.posture}%</div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full mt-2">
            <div className="bg-amber-400 h-full rounded-full" style={{ width: `${dimensions.posture}%` }}></div>
          </div>
        </div>
      </div>

      {/* Intervention Action Hub */}
      <div className="bg-slate-900 border border-slate-800 rounded-xl p-6">
        <h3 className="text-lg font-semibold text-slate-100 mb-4">Guided Wellness Protocols</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="p-4 bg-slate-800/60 rounded-xl border border-slate-700/60">
            <h4 className="font-medium text-slate-200">20-20-20 Visual Rest</h4>
            <p className="text-xs text-slate-400 mt-1 mb-3">Look 20 feet away for 20 seconds to relax eye muscles.</p>
            <button
              onClick={() => completeIntervention("eye")}
              className="w-full py-2 bg-cyan-600 hover:bg-cyan-500 text-white rounded text-xs font-semibold"
            >
              Start 20s Gaze (+15 pts)
            </button>
          </div>

          <div className="p-4 bg-slate-800/60 rounded-xl border border-slate-700/60">
            <h4 className="font-medium text-slate-200">Hydration Check</h4>
            <p className="text-xs text-slate-400 mt-1 mb-3">Log a glass of water (250 ml). Max 1 log per 20 minutes.</p>
            <button
              onClick={() => completeIntervention("water")}
              className="w-full py-2 bg-blue-600 hover:bg-blue-500 text-white rounded text-xs font-semibold"
            >
              Log 250ml Water (+25 pts)
            </button>
          </div>

          <div className="p-4 bg-slate-800/60 rounded-xl border border-slate-700/60">
            <h4 className="font-medium text-slate-200">Box Breathing (2m)</h4>
            <p className="text-xs text-slate-400 mt-1 mb-3">Inhale 4s, hold 4s, exhale 4s, hold 4s to reset stress.</p>
            <button
              onClick={() => completeIntervention("breathe")}
              className="w-full py-2 bg-purple-600 hover:bg-purple-500 text-white rounded text-xs font-semibold"
            >
              Start Breathing (+25 pts)
            </button>
          </div>

          <div className="p-4 bg-slate-800/60 rounded-xl border border-slate-700/60">
            <h4 className="font-medium text-slate-200">Seated Spine Stretch</h4>
            <p className="text-xs text-slate-400 mt-1 mb-3">Gentle torso rotation and shoulder roll (low mobility friendly).</p>
            <button
              onClick={() => completeIntervention("stretch")}
              className="w-full py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded text-xs font-semibold"
            >
              Start Stretch (+25 pts)
            </button>
          </div>

          <div className="p-4 bg-slate-800/60 rounded-xl border border-slate-700/60">
            <h4 className="font-medium text-slate-200">Posture Alignment Reset</h4>
            <p className="text-xs text-slate-400 mt-1 mb-3">Ears over shoulders, feet flat, relax pelvis and wrists.</p>
            <button
              onClick={() => completeIntervention("posture")}
              className="w-full py-2 bg-amber-600 hover:bg-amber-500 text-white rounded text-xs font-semibold"
            >
              Confirm Posture (+20 pts)
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
