const Header = () => {
	return (
		<div>
			<header className="text-white p-4 shadow-md">
				<div className="flex justify-between items-center">
					<h1 className="text-2xl font-semibold"></h1>
					<div className="flex items-center space-x-4">
						<span className="px-2 py-1 bg-blue-500 rounded-md">
							最終更新: 12:34:56
						</span>
						<span className="px-2 py-1 bg-green-500 rounded-md">
							ステータス: オンライン
						</span>
					</div>
				</div>
			</header>
		</div>
	);
};

export default Header;
