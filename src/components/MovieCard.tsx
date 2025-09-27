export interface Movie {
	path: string;
	name: string;
	poster: string;
	year: number;
	category: string;
}

const MovieCard = ({ movie }: { movie: Movie }) => {
	return (
		<div className="max-w-full hover:cursor-pointer w-full relative">
			{
				movie.poster === "placeholder" ?
					<div
						className="bg-cover h-96 p-4 w-full rounded-lg object-contain bg-gray-600"
					><p className="text-white text-xl">{movie.name.substring(0, 50)}</p></div>
					:
					<img
						className="bg-cover h-96 w-full rounded-lg object-contain"
						src={movie.poster}
						alt="movie card"
					/>
			}
			<div className="text-xl text-white mt-4">
				<div className="flex justify-between">
					<h1 className="font-semibold text-md w-full">{movie.name.substring(0, 50)}</h1>
					<p>...</p>
				</div>
				<h1 className="text-gray-400 text-sm font-light">{movie.year}</h1>
			</div>
		</div>
	);
}

export default MovieCard;
