package com.cultivatedsoftware.positivelys;

import android.content.Context;

import com.cultivatedsoftware.positivelys.models.AppRequest;
import com.cultivatedsoftware.positivelys.models.AppResponse;
import com.cultivatedsoftware.positivelys.models.RemindersResponse;
import com.cultivatedsoftware.positivelys.services.AppService;
import com.cultivatedsoftware.positivelys.services.RemindersService;

import java.util.HashMap;

public class RequestResponseMiddleware {

    private final AppService appService;
    private final Context context;

    RequestResponseMiddleware(AppService appService, Context context) {
        this.appService = appService;
        this.context = context;
    }

    public void handle(AppRequest appRequest, AppResponse appResponse) {
        if (appRequest.getUri().equals("https://positivelys.com/reminders") && appRequest.getMethod().toLowerCase().equals("post") && appResponse.getStatus_code() == 302) {
            AppRequest remindersAppRequest = new AppRequest("/reminders", "GET");

            HashMap<String, String> headers = new HashMap<>();
            headers.put("Accept", "application/json");
            remindersAppRequest.setHeaders(headers);

            AppResponse remindersAppResponse = appService.makeRequest(remindersAppRequest);
            RemindersResponse remindersResponse = RemindersResponse.fromJson(remindersAppResponse.getBody());

            RemindersService remindersService = new RemindersService(context);
            remindersService.createReminderNotifications(remindersResponse);
        }
    }
}
