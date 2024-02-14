import { LogicalSize, appWindow } from "@tauri-apps/api/window";

export default function Titlebar() {
	return (
		<div
			data-tauri-drag-region
			className="flex justify-between fixed top-0 left-0 right-0 bg-gray-950 border-y-2 border-gray-800"
		>
			<h1></h1>
			<div className="flex justify-center items-center gap-6 px-6 h-8 text-white bg-emerald-600 rounded-sm hover:cursor-pointer text-md">
				<p>-</p>
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
