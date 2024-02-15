export interface Movie {
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
				<h1 className="font-semibold text-md">{movie.name.substring(0, 50)}</h1>
				<h1 className="text-gray-400 text-sm font-light">{movie.year}</h1>
			</div>
		</div>
	);
}

export default MovieCard;
