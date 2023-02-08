import { invoke } from "@tauri-apps/api/tauri"

export type Device = {
  name: string,
  id: string,
  model: string,
  device: string
  deviceProduct: string
  transportId: string
}

export async function getDevices(): Promise<Device[]> {
  const rawOutput = await invoke("adb_devices_l") as string;  
  const lines = rawOutput.split("\n");
  const deviceLines = lines.slice(1, lines.length).filter(line => line.match(/\S/));

  return deviceLines.map(line => {
    const [,id, deviceProduct, model, device, transportId] = 
      line.match(/(\S+)\s+device product:(\S+) model:(\S+) device:(\S+) transport_id:(\S+)/);
    const name = id;
    return { name, id, model, device, deviceProduct, transportId };
  }); 

}
