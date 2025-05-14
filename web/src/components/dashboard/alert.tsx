const Alert = () => {
	return (
		<>
			<div className="bg-white rounded-lg shadow-md p-4">
				<h2 className="font-semibold text-lg mb-3 flex justify-between">
					<span>アラート</span>
					<span className="bg-red-100 text-red-600 px-2 py-1 rounded-full text-xs">
						2件
					</span>
				</h2>
				<div className="space-y-3">
					<div className="p-3 bg-red-50 border-l-4 border-red-500 rounded-md">
						<div className="text-red-600 font-medium">急速な気温変化</div>
						<div className="text-sm text-gray-600">10分間で-5.2°C</div>
						<div className="text-xs text-gray-500">14:30:02 - 東京</div>
					</div>
					<div className="p-3 bg-yellow-50 border-l-4 border-yellow-500 rounded-md">
						<div className="text-yellow-600 font-medium">気圧の異常変化</div>
						<div className="text-sm text-gray-600">1時間で-8.7hPa</div>
						<div className="text-xs text-gray-500">14:15:30 - 東京</div>
					</div>
					<div className="p-3 hover:bg-gray-50 rounded-md cursor-pointer">
						<div className="text-sm text-blue-500">全てのアラートを表示</div>
					</div>
				</div>
			</div>
		</>
	);
};

export default Alert;
