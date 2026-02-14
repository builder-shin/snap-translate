import styles from "./HotkeyInput.module.css";

interface HotkeyInputProps {
  currentHotkey: string;
}

export default function HotkeyInput({ currentHotkey }: HotkeyInputProps) {
  const displayHotkey = currentHotkey
    .replace("CmdOrCtrl", navigator.platform.includes("Mac") ? "Cmd" : "Ctrl");

  return (
    <div className={styles.container}>
      <label className={styles.label}>단축키</label>
      <div className={styles.hotkeyDisplay}>
        {displayHotkey.split("+").map((key, i) => (
          <span key={i}>
            {i > 0 && <span className={styles.separator}>+</span>}
            <kbd className={styles.key}>{key}</kbd>
          </span>
        ))}
      </div>
      <p className={styles.hint}>단축키 변경은 향후 버전에서 지원됩니다.</p>
    </div>
  );
}
