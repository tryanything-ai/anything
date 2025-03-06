"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

export default function PricingCalculator() {
  const [isDaily, setIsDaily] = useState(false);
  const [numCalls, setNumCalls] = useState(20);
  const [minutesPerCall, setMinutesPerCall] = useState(3);
  const [totalMinutes, setTotalMinutes] = useState(0);
  const [totalCost, setTotalCost] = useState(0);

  // Calculate totals whenever inputs change
  useEffect(() => {
    // Calculate monthly minutes based on frequency
    const daysInMonth = isDaily ? 30 : 4; // 30 days or 4 weeks
    const calculatedMinutes = numCalls * minutesPerCall * daysInMonth;

    // Calculate total cost: (minutes * rate) + base fee
    const minuteRate = 0.5;
    const baseFee = 10;
    const calculatedCost = calculatedMinutes * minuteRate + baseFee;

    setTotalMinutes(calculatedMinutes);
    setTotalCost(calculatedCost);
  }, [isDaily, numCalls, minutesPerCall]);

  return (
    <div className="bg-white rounded-2xl shadow-xl p-8 max-w-3xl mx-auto">
      <div className="space-y-8">
        <div>
          <div className="flex justify-between items-center mb-4">
            <div>
              <h3 className="text-lg font-semibold">Frequency</h3>
              <p className="text-sm text-gray-600">
                How often do you want your AI to make calls?
              </p>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm text-gray-600">Weekly</span>
              <div
                onClick={() => setIsDaily(!isDaily)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2 cursor-pointer ${
                  isDaily ? "bg-purple-600" : "bg-gray-200"
                }`}
                role="switch"
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white shadow-lg transition-transform ${
                    isDaily ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </div>
              <span className="text-sm text-gray-600">Daily</span>
            </div>
          </div>
        </div>

        <div>
          <div className="flex justify-between items-center mb-2">
            <h3 className="text-lg font-semibold">Number of Calls</h3>
            <span className="text-lg font-semibold text-purple-600">
              {numCalls} calls per {isDaily ? "day" : "week"}
            </span>
          </div>
          <div className="relative h-2 w-full rounded-full bg-gray-200">
            <div
              className="absolute h-2 bg-purple-600 rounded-full"
              style={{ width: `${(numCalls / 100) * 100}%` }}
            />
            <input
              type="range"
              min="0"
              max="100"
              step="5"
              value={numCalls}
              onChange={(e) => setNumCalls(Number(e.target.value))}
              className="absolute w-full h-2 opacity-0 cursor-pointer"
            />
          </div>
          <p className="text-sm text-gray-600 mt-2">
            Adjust the number of calls per {isDaily ? "day" : "week"}
          </p>
        </div>

        <div>
          <div className="flex justify-between items-center mb-2">
            <h3 className="text-lg font-semibold">Minutes per Call</h3>
            <span className="text-lg font-semibold text-purple-600">
              {minutesPerCall} minutes
            </span>
          </div>
          <div className="relative h-2 w-full rounded-full bg-gray-200">
            <div
              className="absolute h-2 bg-purple-600 rounded-full"
              style={{ width: `${(minutesPerCall / 10) * 100}%` }}
            />
            <input
              type="range"
              min="1"
              max="10"
              step="1"
              value={minutesPerCall}
              onChange={(e) => setMinutesPerCall(Number(e.target.value))}
              className="absolute w-full h-2 opacity-0 cursor-pointer"
            />
          </div>
          <p className="text-sm text-gray-600 mt-2">
            Average duration of each call
          </p>
        </div>

        <div className="border-t pt-6">
          <div className="flex justify-between items-center mb-4">
            <div>
              <h3 className="text-lg font-semibold">Monthly Usage</h3>
              <p className="text-sm text-gray-600">
                {totalMinutes.toLocaleString()} total minutes
              </p>
            </div>
            <div className="text-right">
              <p className="text-sm text-gray-600">$0.50 per minute</p>
              <p className="text-2xl font-bold text-purple-600">
                $
                {totalCost.toLocaleString(undefined, {
                  minimumFractionDigits: 2,
                  maximumFractionDigits: 2,
                })}
                /month
              </p>
            </div>
          </div>

          <div className="bg-purple-50 rounded-lg p-4 text-sm text-purple-700">
            <p className="font-semibold">Included in your plan:</p>
            <ul className="mt-2 space-y-1">
              <li>• Base platform access ($10/month)</li>
              <li>• Custom AI voice configuration</li>
              <li>• Integrations with your CRM</li>
              <li>• Support</li>
            </ul>
          </div>

          <Link
            href="/get-started"
            className="mt-6 w-full bg-purple-600 text-white px-6 py-3 rounded-lg font-bold hover:bg-purple-700 transition-colors flex items-center justify-center"
          >
            Get Started with AI Calling
          </Link>
        </div>
      </div>
    </div>
  );
}
