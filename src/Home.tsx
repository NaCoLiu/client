import { invoke } from "@tauri-apps/api/core";
import { Button } from "./components/ui/button";

export default function Home() {
    const handleCreateWindow = async () => {
        await invoke('create_windows_frame');
    }
    return <div>
        <Button onClick={handleCreateWindow}>Create Window</Button>
    </div>;
}