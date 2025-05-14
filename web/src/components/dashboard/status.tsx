const Status = () => {
	return (
		<>
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
						<span className="text-red-600 text-sm font-medium">オフライン</span>
					</div>
				</div>
			</div>
		</>
	);
};

export default Status;
