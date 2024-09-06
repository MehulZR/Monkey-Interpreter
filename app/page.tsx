"use client";
import Navbar from "@/components/Navbar";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import dynamic from "next/dynamic";
import Link from "next/link";
import { useState } from "react";

type HomeProps = {
  wasmFn: (input: string) => string;
};

function Home({ wasmFn }: HomeProps) {
  const [inputCode, setInputCode] = useState("");
  const [evaluatedCode, setEvaluatedCode] = useState("");
  const [showTerminal, setShowTerminal] = useState(false);

  const evaluate = async () => {
    try {
      const result = wasmFn(inputCode);
      setEvaluatedCode(result);
    } catch (e) {
      setEvaluatedCode("Syntactical Error");
    }
    setShowTerminal(true);
  };

  return (
    <div className="bg-primary w-screen min-h-screen text-primary">
      <div className="container max-w-screen-lg flex flex-col items-center px-4">
        <Navbar />
        <div className="flex flex-col md:flex-row w-full min-h-[512px] md:min-h-[692px] mt-12 md:mt-16 border border-primary rounded ">
          <div className="md:hidden p-4 flex justify-between items-center border-b border-primary">
            <ToggleGroup
              type="single"
              className="bg-secondary dark:bg-primary p-1 border border-primary rounded"
              defaultValue="editor"
              value={showTerminal ? "terminal" : "editor"}
              onValueChange={(val) =>
                setShowTerminal(val == "terminal" ? true : false)
              }
            >
              <ToggleGroupItem
                value="editor"
                className="data-[state=on]:bg-primary dark:data-[state=on]:bg-secondary-accent"
              >
                Code
              </ToggleGroupItem>

              <ToggleGroupItem
                value="terminal"
                className="data-[state=on]:bg-primary dark:data-[state=on]:bg-secondary-accent"
              >
                Output
              </ToggleGroupItem>
            </ToggleGroup>
            <button
              className="flex gap-1 bg-primary-accent px-4 py-2 rounded justify-center items-center text-white dark:text-cta-text"
              onClick={evaluate}
            >
              Run
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                className="fill-current"
                viewBox="0 0 256 256"
              >
                <path d="M234.49,111.07,90.41,22.94A20,20,0,0,0,60,39.87V216.13a20,20,0,0,0,30.41,16.93l144.08-88.13a19.82,19.82,0,0,0,0-33.86ZM84,208.85V47.15L216.16,128Z"></path>
              </svg>
            </button>
          </div>

          <div
            className={`${showTerminal ? "hidden" : "flex"} md:flex flex-col flex-grow w-full`}
          >
            <p className="hidden md:block p-4 border-b border-primary">Input</p>
            <textarea
              className="bg-secondary dark:bg-primary p-4 border-b border-primary flex-grow w-full focus:outline-none"
              value={inputCode}
              onChange={(e) => setInputCode(e.target.value)}
            ></textarea>
            <div className="hidden md:flex items-center justify-between p-4">
              <p className="text-placeholder">
                Made with ♥️ by{" "}
                <Link
                  href="https://github.com/mehulzr"
                  target="_blank"
                  className="underline"
                >
                  Mehul
                </Link>
              </p>
              <button
                className="flex gap-1 bg-primary-accent px-4 py-2 rounded justify-center items-center text-white dark:text-cta-text"
                onClick={evaluate}
              >
                Run
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  className="fill-current"
                  viewBox="0 0 256 256"
                >
                  <path d="M234.49,111.07,90.41,22.94A20,20,0,0,0,60,39.87V216.13a20,20,0,0,0,30.41,16.93l144.08-88.13a19.82,19.82,0,0,0,0-33.86ZM84,208.85V47.15L216.16,128Z"></path>
                </svg>
              </button>
            </div>
          </div>

          <div
            className={`${showTerminal ? "flex" : "hidden"} md:flex flex-col w-full bg-terminal flex-grow`}
          >
            <p className="hidden md:block p-4 text-terminal-placeholder border-b border-terminal">
              Output
            </p>
            <p className="p-4 text-terminal flex-grow">{evaluatedCode}</p>
          </div>
        </div>
      </div>
    </div>
  );
}

const DynamicComponent = dynamic(
  async () => {
    const wasmModule = await import("monkey_interpreter");

    const HomeWrapper = () => <Home wasmFn={wasmModule.interpret} />;

    return HomeWrapper;
  },
  {
    ssr: false,
  },
);

export default DynamicComponent;
