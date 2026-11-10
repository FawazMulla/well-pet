import React, { useState } from "react";
import { PetSpecies, EmotionalState } from "../App";

interface PetOverlayProps {
  name: string;
  species: PetSpecies;
  state: EmotionalState;
  compositeScore: number;
}

export const PetOverlay: React.FC<PetOverlayProps> = ({
  name,
  species,
  state,
  compositeScore,
}) => {
  const [position, setPosition] = useState({ x: 20, y: 20 });
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });

  const getSpriteEmoji = () => {
    switch (species) {
      case "cat":
        if (state === "napping") return "😴💤";
        if (state === "sleepy") return "🥱🐱";
        if (state === "tired") return "🐱💧";
        if (state === "happy") return "😺✨";
        return "😻💖"; // thriving
      case "dog":
        if (state === "napping") return "😴💤";
        if (state === "sleepy") return "🥱🐶";
        if (state === "tired") return "🐶💧";
        if (state === "happy") return "🐶🎾";
        return "🐶🌟";
      case "plant":
        if (state === "napping") return "🌱💤";
        if (state === "sleepy") return "🥀";
        if (state === "tired") return "🪴💧";
        if (state === "happy") return "🌿✨";
        return "🌸🌟";
    }
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    setDragOffset({
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging) {
      setPosition({
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  return (
    <div
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      style={{
        position: "fixed",
        right: `${position.x}px`,
        bottom: `${position.y}px`,
        zIndex: 9999,
      }}
      className="select-none"
    >
      <div
        onMouseDown={handleMouseDown}
        className="cursor-move bg-slate-900/90 hover:bg-slate-900 border border-emerald-500/40 backdrop-blur-md rounded-2xl p-3 shadow-2xl transition flex flex-col items-center justify-center min-w-[120px]"
      >
        <div className="text-3xl mb-1 filter drop-shadow-md animate-bounce duration-1000">
          {getSpriteEmoji()}
        </div>
        <div className="text-xs font-semibold text-slate-200">{name}</div>
        <div className="flex items-center gap-1.5 mt-1">
          <span
            className={`w-2 h-2 rounded-full ${
              state === "thriving"
                ? "bg-emerald-400"
                : state === "happy"
                ? "bg-blue-400"
                : state === "tired"
                ? "bg-amber-400"
                : state === "sleepy"
                ? "bg-orange-400"
                : "bg-purple-400"
            }`}
          ></span>
          <span className="text-[10px] uppercase font-bold text-slate-400 tracking-wider">
            {state}
          </span>
        </div>
        <div className="text-[10px] text-emerald-400 mt-0.5">{compositeScore}% S</div>
      </div>
    </div>
  );
};
