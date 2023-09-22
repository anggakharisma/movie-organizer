import { useEffect, useState } from "react";
import { open } from "@tauri-apps/api/dialog";

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { appWindow } from "@tauri-apps/api/window";

interface Movie {
	path: string;
	name: string;
	poster: string;
	year: number;
	category: string;
}

const MovieCard = ({ movie }: { movie: Movie }) => {
	return (
		<div className="max-w-full hover:cursor-pointer w-full">
			<img
				className="bg-cover rounded-lg"
				src={movie.poster}
				alt="movie card"
			/>
			<div className="text-xl text-white mt-4">
				<h1 className="font-semibold text-2xl">{movie.name}</h1>
				<h1 className="text-gray-400 font-light">{movie.year}</h1>
			</div>
		</div>
	);
};

function App() {
	const [movieList, setMovieList] = useState<any>([]);
	const [error, setError] = useState<string>("");

	useEffect(() => {
		fetchMovieLists();
	}, []);

	async function fetchMovieLists() {
		try {
			await setMovieList(await invoke("get_movie_list"));
		} catch (e) {
			if (typeof e === "string") {
				setError(e);
			}
		}
	}

	return (
		<div className="min-h-max">
			<div
				data-tauri-drag-region
				className="flex justify-between gap-4 align-middle items-center w-[95%] mt-8 m-auto"
			>
				<div className="text-white pb-2">
					<h1 className="text-emerald-500 font-semibold tracking-widest text-4xl">
						SEIRI
					</h1>
					<p className="text-sm">Movie Organizer</p>
				</div>
				<div className="flex justify-center gap-10">
					<p className="text-white font-normal text-md p-2 text-center">
						Config
					</p>
					<p
						onClick={() => appWindow.close()}
						className="text-white font-normal text-md p-2 text-center"
					>
						Refresh
					</p>
				</div>
			</div>

			{/* This is sucks change this, dont like how this is handled */}
			<div className="bg-yellow-400">
				{error !== "" && (
					<div>
						<h2 className="text-lg">
							<span className="bg-black text-xl inline-block py-2 text-yellow-400 px-4 mr-1">
								!
							</span>{" "}
							{error && error}{" "}
						</h2>
						<button
							type="button"
							onClick={async () => {
								const selected = await open({
									directory: true,
									multiple: false,
								});

								if (selected === null) return;

								await invoke("set_selected_dir", {
									dir: selected,
								});

								await setMovieList(await invoke("get_movie_list"));
								setError("");
							}}
							className="ml-12 bg-black text-white mb-2 px-4 py-2 mt-2"
						>
							Set directory
						</button>
					</div>
				)}
			</div>

			<div className="w-10/12 max-w-full mt-8 pt-2 py-8 px-10 m-auto">
				<h1 className="text-2xl text-white my-4">Your movies</h1>
				<div className="grid grid-cols-5 gap-8">
					{movieList.map((item: Movie, id: number) => (
						<MovieCard key={id} movie={item} />
					))}
				</div>
			</div>
		</div>
	);
}

export default App;
