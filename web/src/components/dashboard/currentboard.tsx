const CurrentBoard = () => {
	return (
		<div className="bg-white rounded-lg shadow-md p-4">
			<h2 className="font-semibold text-xl mb-4">東京 - 現在の気象状況</h2>
			<div className="grid grid-cols-3 gap-4">
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">気温</div>
					<div className="text-3xl font-bold text-blue-600">22.5°C</div>
				</div>
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">湿度</div>
					<div className="text-3xl font-bold text-blue-600">65.3%</div>
				</div>
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">気圧</div>
					<div className="text-3xl font-bold text-blue-600">1013.2hPa</div>
				</div>
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">風向</div>
					<div className="text-3xl font-bold text-blue-600">北東</div>
				</div>
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">風速</div>
					<div className="text-3xl font-bold text-blue-600">4.2m/s</div>
				</div>
				<div className="p-3 bg-blue-50 rounded-lg text-center">
					<div className="text-sm text-gray-500">降水量</div>
					<div className="text-3xl font-bold text-blue-600">0.0mm</div>
				</div>
			</div>
		</div>
	);
};

export default CurrentBoard;
