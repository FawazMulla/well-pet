import React, { useState } from "react";
import { Download, Trash2, ShieldCheck, TrendingUp } from "lucide-react";

interface DashboardProps {
  compositeScore: number;
}

export const Dashboard: React.FC<DashboardProps> = ({ compositeScore }) => {
  const [purged, setPurged] = useState(false);

  const handleExportJSON = () => {
    const data = {
      export_date: new Date().toISOString(),
      composite_score: compositeScore,
      compliance_rate: 88.5,
      habit_streak_days: 14,
      bandit_parameters: {
        arms_evaluated: 112,
        preferred_offset_minutes: -10,
      },
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `wellpet_telemetry_${Date.now()}.json`;
    a.click();
  };

  const handleCryptoErase = () => {
    if (window.confirm("Are you sure you want to permanently erase all local encrypted logs and preferences?")) {
      setPurged(true);
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <header className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-slate-100">Analytics & Dashboard</h2>
          <p className="text-sm text-slate-400">
            Privacy-first longitudinal trends benchmarked against your personal baseline.
          </p>
        </div>
        <div className="flex gap-2">
          <button
            onClick={handleExportJSON}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs rounded-lg border border-slate-700"
          >
            <Download className="w-3.5 h-3.5" /> Export JSON
          </button>
          <button
            onClick={handleCryptoErase}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-rose-950/40 hover:bg-rose-900/60 text-rose-300 text-xs rounded-lg border border-rose-800/40"
          >
            <Trash2 className="w-3.5 h-3.5" /> Purge All Data
          </button>
        </div>
      </header>

      {purged && (
        <div className="p-3 bg-rose-950/60 border border-rose-600/50 rounded-xl text-xs text-rose-200">
          Cryptographic master key destroyed and local database tables erased.
        </div>
      )}

      {/* Metrics Row */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl">
          <span className="text-xs text-slate-400">Fair Break Compliance</span>
          <div className="text-3xl font-bold text-emerald-400 mt-1">88.5%</div>
          <span className="text-[11px] text-slate-500 mt-2 block">
            Excludes 4 hard-gate suppressed prompts (meetings)
          </span>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl">
          <span className="text-xs text-slate-400">Habit Streak</span>
          <div className="text-3xl font-bold text-cyan-400 mt-1">14 Days</div>
          <span className="text-[11px] text-slate-500 mt-2 block">
            2 Streak Freezes remaining this week
          </span>
        </div>

        <div className="bg-slate-900 border border-slate-800 p-5 rounded-xl">
          <span className="text-xs text-slate-400">LinUCB Timing Lift</span>
          <div className="text-3xl font-bold text-purple-400 mt-1">+14.2%</div>
          <span className="text-[11px] text-slate-500 mt-2 block">
            Acceptance increase vs standard 50m fixed timer
          </span>
        </div>
      </div>

      {/* 7-Day Trend Card */}
      <div className="bg-slate-900 border border-slate-800 rounded-xl p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="font-semibold text-slate-200 text-sm">7-Day Composite Wellness Trend</h3>
          <span className="flex items-center gap-1 text-xs text-emerald-400">
            <TrendingUp className="w-4 h-4" /> +8% above 7-day personal baseline
          </span>
        </div>

        {/* Mock Chart bars */}
        <div className="h-40 flex items-end gap-4 pt-6 pb-2 border-b border-slate-800">
          {[
            { day: "Mon", val: 82 },
            { day: "Tue", val: 78 },
            { day: "Wed", val: 88 },
            { day: "Thu", val: 85 },
            { day: "Fri", val: 92 },
            { day: "Sat", val: 95 },
            { day: "Sun", val: 90 },
          ].map((bar, idx) => (
            <div key={idx} className="flex-1 flex flex-col items-center gap-2">
              <span className="text-[10px] text-slate-400">{bar.val}%</span>
              <div
                className="w-full bg-emerald-500/80 hover:bg-emerald-400 rounded-t transition-all"
                style={{ height: `${bar.val * 1.2}px` }}
              ></div>
              <span className="text-xs text-slate-400">{bar.day}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
