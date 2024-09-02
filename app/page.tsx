"use client";
import dynamic from "next/dynamic";
import { useState } from "react";

type HomeProps = {
  wasmFn: (input: string) => string;
};

const Home = ({ wasmFn }: HomeProps) => {
  const [inputCode, setInputCode] = useState("");
  const [evaluatedCode, setEvaluatedCode] = useState("");

  return (
    <div>
      <input value={inputCode} onChange={(e) => setInputCode(e.target.value)} />
      <button onClick={() => setEvaluatedCode(wasmFn(inputCode))}>Run</button>
      <p>{evaluatedCode}</p>
    </div>
  );
};

export default dynamic(
  async () => {
    const wasmModule = await import("monkey_interpreter");

    return () => <Home wasmFn={wasmModule.interpret} />;
  },
  { ssr: false },
);
