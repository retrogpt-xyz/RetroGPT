import { MainScreenInner } from "./MainScreenInner";

export const MainScreen = () => {
  return (
    <div
      style={{
        width: "100vw",
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <MainScreenInner />
    </div>
  );
}; 