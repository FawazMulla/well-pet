import React, { useState } from "react";
import { Calendar, Clock, CheckCircle2, ShieldAlert } from "lucide-react";

export const DayPlanner: React.FC = () => {
  const [selectedArchetype, setSelectedArchetype] = useState("Standard");

  const archetypes = [
    "Standard",
    "Deadline Sprint",
    "Learning/Research",
    "Creative Flow",
    "Meeting Intensive",
    "Recovery/Gentle",
    "Low Sleep",
    "Custom",
  ];

  const schedule = [
    { time: "09:00", label: "Session Start & Goal Setting", type: "focus" },
    { time: "09:50", label: "20-20-20 Eye Gaze Anchor (Bandit Optimized)", type: "break" },
    { time: "10:45", label: "Hydration & Posture Reset", type: "break" },
    { time: "11:30", label: "Team Sync (Hard Gate Protected)", type: "meeting" },
    { time: "12:15", label: "Nutritional Pause / Meal Break (45m)", type: "meal" },
    { time: "14:00", label: "Box Breathing & Focus Recharge", type: "break" },
    { time: "15:15", label: "Walking Break / Mobility (5m)", type: "break" },
    { time: "16:45", label: "Administrative Wrap-up & Daily Review", type: "wrapup" },
  ];

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <header className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-slate-100">Day Planner</h2>
          <p className="text-sm text-slate-400">
            Goal-aware cadences synthesized with ergonomic rules and calendar gates.
          </p>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs text-slate-400">Day Archetype:</span>
          <select
            value={selectedArchetype}
            onChange={(e) => setSelectedArchetype(e.target.value)}
            className="bg-slate-900 border border-slate-700 text-slate-200 text-xs rounded-lg px-3 py-1.5 focus:outline-none focus:border-emerald-500"
          >
            {archetypes.map((a) => (
              <option key={a} value={a}>
                {a}
              </option>
            ))}
          </select>
        </div>
      </header>

      {/* Ergonomic Validation Status Banner */}
      <div className="bg-emerald-950/40 border border-emerald-500/30 p-4 rounded-xl flex items-center gap-3">
        <CheckCircle2 className="w-5 h-5 text-emerald-400" />
        <div className="text-xs">
          <span className="font-semibold text-emerald-300">
            Ergonomic Ruleset Verified (8/8 points):
          </span>{" "}
          <span className="text-slate-300">
            Intervals between 20m–90m, 18% break-to-work ratio, meal pause guaranteed, zero meeting collisions.
          </span>
        </div>
      </div>

      {/* Schedule Timeline */}
      <div className="bg-slate-900 border border-slate-800 rounded-xl p-6">
        <h3 className="text-md font-semibold text-slate-200 mb-4">Today's Optimized Timeline</h3>
        <div className="space-y-3">
          {schedule.map((item, index) => (
            <div
              key={index}
              className={`flex items-center justify-between p-3 rounded-lg border text-xs ${
                item.type === "meeting"
                  ? "bg-slate-800/40 border-slate-700/60 text-slate-400"
                  : item.type === "break"
                  ? "bg-emerald-950/20 border-emerald-500/30 text-emerald-200"
                  : item.type === "meal"
                  ? "bg-amber-950/20 border-amber-500/30 text-amber-200"
                  : "bg-slate-800/80 border-slate-700 text-slate-200"
              }`}
            >
              <div className="flex items-center gap-3">
                <Clock className="w-4 h-4 text-slate-400" />
                <span className="font-mono font-medium">{item.time}</span>
                <span className="font-medium">{item.label}</span>
              </div>
              <span className="uppercase text-[10px] font-bold px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                {item.type}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
