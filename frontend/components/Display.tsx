import { DisplayMsg } from "../App";

interface Props {
  messages: DisplayMsg[];
}

export const Display = ({ messages }: Props) => {
  return messages.map(({source, msg}) => {
    return (
      <>
        <p
          style={{
            font: "Static JetBrains Mono",
            fontSize: "19px"
          }}
        >
          <span style={{ color: "green" }}>[{source}]</span>: {msg}
        </p>
      </>
    );
  });
};
