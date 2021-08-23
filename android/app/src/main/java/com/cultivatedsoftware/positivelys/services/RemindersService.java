package com.cultivatedsoftware.positivelys.services;

import android.content.Context;

import com.cultivatedsoftware.positivelys.models.Reminder;
import com.cultivatedsoftware.positivelys.models.RemindersResponse;

import java.util.Calendar;

public class RemindersService {
    private final Context context;

    public RemindersService(Context context) {
        this.context = context;
    }

    public void createReminderNotifications(RemindersResponse remindersResponse) {
        NotificationService notificationService = new NotificationService(context);
        notificationService.removeAllNotifications();

        for (Reminder reminder : remindersResponse.reminders) {
            switch (reminder.day) {
                case "Everyday":
                    notificationService.createNotification(Calendar.SUNDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.MONDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.TUESDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.WEDNESDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.THURSDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.FRIDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.SATURDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Weekdays":
                    notificationService.createNotification(Calendar.MONDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.TUESDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.WEDNESDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.THURSDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.FRIDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Weekends":
                    notificationService.createNotification(Calendar.SUNDAY, reminder.hour, reminder.minute, reminder.id);
                    notificationService.createNotification(Calendar.SATURDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Sunday":
                    notificationService.createNotification(Calendar.SUNDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Monday":
                    notificationService.createNotification(Calendar.MONDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Tuesday":
                    notificationService.createNotification(Calendar.TUESDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Wednesday":
                    notificationService.createNotification(Calendar.WEDNESDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Thursday":
                    notificationService.createNotification(Calendar.THURSDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Friday":
                    notificationService.createNotification(Calendar.FRIDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
                case "Saturday":
                    notificationService.createNotification(Calendar.SATURDAY, reminder.hour, reminder.minute, reminder.id);
                    break;
            }
        }
    }
}
