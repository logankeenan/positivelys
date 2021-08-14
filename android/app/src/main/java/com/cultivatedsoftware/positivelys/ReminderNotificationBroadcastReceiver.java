package com.cultivatedsoftware.positivelys;

import android.app.Notification;
import android.app.NotificationManager;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;

import com.cultivatedsoftware.positivelys.services.NotificationService;

import static com.cultivatedsoftware.positivelys.services.NotificationService.REMINDER_ID_KEY;


public class ReminderNotificationBroadcastReceiver extends BroadcastReceiver {
    @Override
    public void onReceive(final Context context, Intent intent) {
        NotificationManager notificationManager = (NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE);

        int notificationId = intent.getIntExtra(REMINDER_ID_KEY, 0);
        if (notificationId != 0) {
            
            Notification reminderNotification = new NotificationService(context).createReminderNotification();
            notificationManager.notify(notificationId , reminderNotification);
        }
    }
}
