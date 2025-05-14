const Measurements = () => {
	return (
		<>
			<div className="bg-white rounded-lg shadow-md p-4">
				<h2 className="font-semibold text-lg mb-4">
					温度・湿度の推移（過去1時間）
				</h2>
				<div className="h-64 bg-gray-100 rounded-md flex items-center justify-center">
					<div className="text-gray-400">温度・湿度グラフが表示されます</div>
				</div>
			</div>
		</>
	);
};

export default Measurements;
