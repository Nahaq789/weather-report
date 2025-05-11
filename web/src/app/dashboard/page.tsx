import React from "react";

const DashboardLayout = () => {
	return (
		<div className="flex flex-col w-full h-screen bg-gray-50 text-gray-800 overflow-auto">
			<header className="bg-blue-600 text-white p-4 shadow-md">
				<div className="flex justify-between items-center">
					<h1 className="text-2xl font-semibold">気象データ配信サービス</h1>
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

			<main className="flex-grow p-4 grid grid-cols-12 gap-4">
				// sidebar
				<div className="col-span-7 space-y-4">
					<div className="bg-white rounded-lg shadow-md p-4">
						<h2 className="font-semibold text-xl mb-4">
							東京 - 現在の気象状況
						</h2>
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
								<div className="text-3xl font-bold text-blue-600">
									1013.2hPa
								</div>
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

					<div className="bg-white rounded-lg shadow-md p-4">
						<h2 className="font-semibold text-lg mb-4">
							温度・湿度の推移（過去1時間）
						</h2>
						<div className="h-64 bg-gray-100 rounded-md flex items-center justify-center">
							<div className="text-gray-400">
								温度・湿度グラフが表示されます
							</div>
						</div>
					</div>

					<div className="bg-white rounded-lg shadow-md p-4">
						<h2 className="font-semibold text-lg mb-4">
							気圧変化（過去3時間）
						</h2>
						<div className="h-48 bg-gray-100 rounded-md flex items-center justify-center">
							<div className="text-gray-400">気圧変化グラフが表示されます</div>
						</div>
					</div>
				</div>
				<div className="col-span-3 space-y-4">
					<div className="bg-white rounded-lg shadow-md p-4">
						<h2 className="font-semibold text-lg mb-3">風向・風速</h2>
						<div className="h-64 bg-gray-100 rounded-md flex items-center justify-center">
							<div className="text-gray-400">風配図が表示されます</div>
						</div>
					</div>

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
								<div className="text-yellow-600 font-medium">
									気圧の異常変化
								</div>
								<div className="text-sm text-gray-600">1時間で-8.7hPa</div>
								<div className="text-xs text-gray-500">14:15:30 - 東京</div>
							</div>
							<div className="p-3 hover:bg-gray-50 rounded-md cursor-pointer">
								<div className="text-sm text-blue-500">
									全てのアラートを表示
								</div>
							</div>
						</div>
					</div>

					<div className="bg-white rounded-lg shadow-md p-4">
						<h2 className="font-semibold text-lg mb-3">センサーステータス</h2>
						<div className="space-y-2">
							<div className="flex justify-between items-center px-3 py-2 bg-green-50 rounded-md">
								<span>sensor-tokyo-1</span>
								<span className="text-green-600 text-sm font-medium">正常</span>
							</div>
							<div className="flex justify-between items-center px-3 py-2 bg-green-50 rounded-md">
								<span>sensor-tokyo-2</span>
								<span className="text-green-600 text-sm font-medium">正常</span>
							</div>
							<div className="flex justify-between items-center px-3 py-2 bg-red-50 rounded-md">
								<span>sensor-tokyo-3</span>
								<span className="text-red-600 text-sm font-medium">
									オフライン
								</span>
							</div>
						</div>
					</div>
				</div>
			</main>

			<footer className="bg-gray-100 p-3 text-center text-sm text-gray-500">
				天気データ配信サービス © 2025
			</footer>
		</div>
	);
};

export default DashboardLayout;
