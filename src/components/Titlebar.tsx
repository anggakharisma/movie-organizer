import { LogicalSize, PhysicalSize, appWindow } from "@tauri-apps/api/window";

export default function Titlebar() {
	return (
		<div
			data-tauri-drag-region
			className="flex justify-between fixed top-0 left-0 right-0 bg-gray-950 border-y-2 border-gray-800"
		>
			<h1>Title</h1>
			<div className="flex gap-8 text-white bg-emerald-600 py-1 px-6 rounded-md mr-2 hover:cursor-pointer text-md">
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
