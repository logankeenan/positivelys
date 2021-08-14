package com.cultivatedsoftware.positivelys.models;

import com.google.gson.Gson;

import java.util.List;

public class RemindersResponse {
    public List<Reminder> reminders;

    public static RemindersResponse fromJson(String json) {
        Gson gson = new Gson();

        return gson.fromJson(json, RemindersResponse.class);
    }
}