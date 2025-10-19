import { LogicalSize } from "@tauri-apps/api/window";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

export default function Titlebar() {
	return (
		<div
			data-tauri-drag-region
			className="flex justify-between  items-center align-center fixed top-0 left-0 right-0 bg-gray-950 border-y-2 border-gray-800 z-50"
		>
			<h1 className="text-emerald-600 text-md font-bold ml-4"></h1>
			<div className="flex justify-center align-center items-center gap-6 px-4 text-white bg-emerald-600 rounded-sm hover:cursor-pointer text-md">
				<p onClick={async () => {
					await appWindow.minimize();
				}}>-</p>
				<p
					onClick={async () => {
						if (await appWindow.isMaximized()) {
							await appWindow.setSize(new LogicalSize(1280, 720));
							return;
						}
						await appWindow.maximize();
					}}
				>
					O
				</p>
				<p onClick={() => appWindow.close()}>X</p>
			</div>
		</div>
	);
}
