import { StaticJetBrainsMono } from "./components/StaticJetBrainsMono";
import { Display } from "./components/Display";
import { InputBox } from "./components/InputBox";
import { useState } from "react";

export interface DisplayMsg {
  source: string;
  msg: string;
}

const App = () => {
  let [msgs, setMsgs] = useState<DisplayMsg[]>([]);

  const handleSubmit = async (prompt: string) => {
    let newMsgs = [...msgs, { source: "USER", msg: prompt }];
    setMsgs(newMsgs);

    let resp = await fetch("/api/prompt", {
      method: "POST",
      body: JSON.stringify(newMsgs),
    });

    setMsgs([...newMsgs, { source: "RETROGPT", msg: await resp.text() }]);
  };

  return (
    <>
      <StaticJetBrainsMono />
      <Display messages={msgs} />
      <InputBox onSubmit={handleSubmit} />
    </>
  );
};

export default App;
