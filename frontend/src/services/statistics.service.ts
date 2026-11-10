import api from "@/api/axios";
import type { EmailPreferenceStats, InterestPopularity } from "@/types/statistics.types";

class StatisticsService {
  async getInterestPopularity(): Promise<InterestPopularity[]> {
    const { data } = await api.get<InterestPopularity[]>("/api/statistics/interests/popularity");
    return data;
  }

  async exportInterestPopularity(): Promise<void> {
    const response = await api.get("/api/statistics/interests/popularity/export", {
      params: {},
      responseType: "blob",
    });
    const url = URL.createObjectURL(response.data);
    const a = document.createElement("a");
    a.href = url;
    a.download = "interest-popularity.xlsx";
    a.click();
    URL.revokeObjectURL(url);
  }

  async getEmailPreferenceStats(): Promise<EmailPreferenceStats> {
    const { data } = await api.get<EmailPreferenceStats>(
      "/api/statistics/members/email-preference",
    );
    return data;
  }
}

export const statisticsService = new StatisticsService();
