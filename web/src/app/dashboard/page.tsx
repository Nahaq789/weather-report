"use client";

import Alert from "@/components/dashboard/alert";
import CurrentBoard from "@/components/dashboard/currentboard";
import Measurements from "@/components/dashboard/measurements";
import Status from "@/components/dashboard/status";
import Header from "@/components/Header";
import Sidebar from "@/components/Sidebar";
import React from "react";

const DashboardLayout = () => {
  return (
    <div className="flex flex-col w-full min-h-screen bg-gray-50 text-gray-800">
      <Header />
      <main className="flex-grow p-4">
        <div className="grid grid-cols-12 gap-4 h-full">
          <Sidebar />

          <div className="col-span-7 flex flex-col space-y-4 overflow-y-auto min-h-0">
            <div className="flex-shrink-0">
              <CurrentBoard />
            </div>
            <div className="flex-grow min-h-0">
              <Measurements />
            </div>
          </div>

          <div className="col-span-3 flex flex-col space-y-4 h-fit sticky top-4 overflow-y-auto max-h-[calc(100vh-2rem)]">
            <Alert />
            <Status />
          </div>
        </div>
      </main>
    </div>
  );
};

export default DashboardLayout;
