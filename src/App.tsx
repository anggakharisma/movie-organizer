import { useEffect, useState } from "react";
import { ask, open } from "@tauri-apps/api/dialog";

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Movie {
	path: string;
	name: string;
	poster: string;
	year: number;
}

const MovieCard = ({ movie }: { movie: Movie }) => {
	return (
		<div className="max-w-full w-[15%]">
			<img
				className="bg-cover"
				src="https://movies.universalpictures.com/media/opr-tsr1sheet3-look2-rgb-3-1-1-64545c0d15f1e-1.jpg"
				alt="movie card"
			/>
			<div className="text-xl text-white mt-4">
				<h1 className="">{movie.name}</h1>
				<h1>{movie.year}</h1>
			</div>
		</div>
	);
};

function App() {
	const [movieList, setMovieList] = useState<any>([]);
	const [error, setError] = useState<String>("");

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
		<div className="min-h-screen">
			<div className="flex w-full justify-between align-middle items-center">
				<div className="text-white inline-block bg-emerald-600 px-8 py-2">
					<h1 className="tracking-[.25rem] font-semibold text-2xl">SEIRI</h1>
					<p className="text-sm">Movie Organizer</p>
				</div>
				<p className="mr-8 bg-orange-600 text-white font-bold p-2">O</p>
			</div>

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
			<div className="flex flex-wrap w-10/12 mt-8 m-auto gap-8">
				{movieList.map((item: Movie, id: number) => (
					<MovieCard key={id} movie={item} />
				))}
			</div>
		</div>
	);
}

export default App;
