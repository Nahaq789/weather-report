import Alert from "@/components/dashboard/alert";
import CurrentBoard from "@/components/dashboard/currentboard";
import Measurements from "@/components/dashboard/measurements";
import Status from "@/components/dashboard/status";
import Header from "@/components/Header";
import Sidebar from "@/components/Sidebar";
import React from "react";

const DashboardLayout = () => {
  return (
    <div className="flex flex-col w-full h-screen bg-gray-50 text-gray-800 overflow-auto">
      <Header />
      <main className="flex-grow p-4 grid grid-cols-12 gap-4">
        <Sidebar />
        <div className="col-span-7 space-y-4">
          <CurrentBoard />
          <Measurements />
        </div>
        <div className="col-span-3 space-y-4">
          <Alert />
          <Status />
        </div>
      </main>
    </div>
  );
};

export default DashboardLayout;
