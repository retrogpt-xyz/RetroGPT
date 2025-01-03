import jetBrainsMono from "../assets/JetBrainsMono-Regular.ttf";

export const StaticJetBrainsMono = () => {
  return (
    <style>
      {`
        @font-face {
          font-family: 'Static JetBrains Mono';
          src: url(${jetBrainsMono}) format('truetype');
          font-weight: normal;
          font-style: normal;
        }
      `}
    </style>
  );
};
