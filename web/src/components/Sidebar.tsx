const Sidebar = () => {
	return (
		<div>
			<div className="col-span-2 bg-white rounded-lg shadow-md p-4">
				<h2 className="font-semibold text-lg mb-4 border-b pb-2">観測地点</h2>
				<ul className="space-y-2">
					<li className="px-3 py-2 bg-blue-100 rounded-md font-medium cursor-pointer">
						東京
					</li>
					<li className="px-3 py-2 hover:bg-gray-100 rounded-md cursor-pointer">
						大阪
					</li>
					<li className="px-3 py-2 hover:bg-gray-100 rounded-md cursor-pointer">
						札幌
					</li>
					<li className="px-3 py-2 hover:bg-gray-100 rounded-md cursor-pointer">
						福岡
					</li>
					<li className="px-3 py-2 hover:bg-gray-100 rounded-md cursor-pointer">
						沖縄
					</li>
				</ul>

				<h2 className="font-semibold text-lg mt-6 mb-4 border-b pb-2">設定</h2>
				<div className="space-y-4">
					<div>
						<label className="block text-sm mb-1">表示単位</label>
						<select className="w-full p-2 border rounded-md bg-gray-50">
							<option>摂氏 (°C)</option>
							<option>華氏 (°F)</option>
						</select>
					</div>
					<div>
						<label className="block text-sm mb-1">更新間隔</label>
						<select className="w-full p-2 border rounded-md bg-gray-50">
							<option>5秒</option>
							<option>10秒</option>
							<option>30秒</option>
							<option>1分</option>
						</select>
					</div>
				</div>
			</div>
		</div>
	);
};
