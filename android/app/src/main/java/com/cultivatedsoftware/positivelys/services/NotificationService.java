package com.cultivatedsoftware.positivelys.services;

import android.app.AlarmManager;
import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.drawable.BitmapDrawable;
import android.graphics.drawable.Drawable;
import android.os.Build;

import androidx.core.app.NotificationCompat;

import com.cultivatedsoftware.positivelys.AppPageActivity;
import com.cultivatedsoftware.positivelys.R;
import com.cultivatedsoftware.positivelys.ReminderNotificationBroadcastReceiver;

import java.util.Calendar;

import static java.util.Calendar.AM_PM;
import static java.util.Calendar.DAY_OF_WEEK;

public class NotificationService {
    public static String REMINDER_ID_KEY = "REMINDER_ID_KEY";
    public static Integer ALARM_REQUEST_CODE = 1;
    private static final String SHARED_PREFERENCES_NOTIFICATIONS_KEY = "SHARED_PREFERENCES_NOTIFICATIONS_KEY";
    private static final String NUMBER_OF_NOTIFICATIONS_KEY = "numberOfReminderNotifications";
    public static String NOTIFICATION_CHANNEL_NAME = "REMINDERS_CHANNEL_NAME";
    public static String NOTIFICATION_CHANNEL_ID = "REMINDERS_CHANNEL_ID";
    public static String NOTIFICATION_CHANNEL_DESCRIPTION = "Reminder to create Positivelys";
    public static Integer NOTIFICATION_REQUEST_CODE = 2;

    private final Context context;

    public NotificationService(Context context) {
        this.context = context;
    }

    public void createNotification(Integer day, Integer hour, Integer minute, Integer id) {
        Intent notificationIntent = createNotificationIntent(null);
        PendingIntent pendingIntent = getPendingIntent(notificationIntent);

        AlarmManager alarmManager = (AlarmManager) context.getSystemService(Context.ALARM_SERVICE);
        long firstAlarmStartTime = getTriggerTimeInMilliseconds(hour, minute, day);
        alarmManager.setInexactRepeating(AlarmManager.RTC_WAKEUP, firstAlarmStartTime, AlarmManager.INTERVAL_DAY * 7, pendingIntent);
    }

    public void removeAllNotifications() {
        SharedPreferences sharedPreferences = this.context.getSharedPreferences(SHARED_PREFERENCES_NOTIFICATIONS_KEY, Context.MODE_PRIVATE);
        int numberOfNotifications = sharedPreferences.getInt(NUMBER_OF_NOTIFICATIONS_KEY, 0);

        int max = numberOfNotifications + 1;
        for (int id = 0; id < max; id++) {
            AlarmManager alarmManager = (AlarmManager) context.getSystemService(Context.ALARM_SERVICE);

            Intent notificationIntent = createNotificationIntent(id);
            PendingIntent pendingIntent = getPendingIntent(notificationIntent);

            alarmManager.cancel(pendingIntent);
        }
    }

    private PendingIntent getPendingIntent(Intent notificationIntent) {
        return PendingIntent.getBroadcast(
                context, ALARM_REQUEST_CODE, notificationIntent, PendingIntent.FLAG_UPDATE_CURRENT);
    }


    private Intent createNotificationIntent(Integer id) {
        if (id == null) {
            id = getIdForNotification();
        }

        Intent intent = new Intent(context, ReminderNotificationBroadcastReceiver.class);
        intent.putExtra(REMINDER_ID_KEY, id);
        intent.setAction(id.toString());
        return intent;
    }

    private Integer getIdForNotification() {
        SharedPreferences sharedPreferences = this.context.getSharedPreferences(SHARED_PREFERENCES_NOTIFICATIONS_KEY, Context.MODE_PRIVATE);
        int id = sharedPreferences.getInt(NUMBER_OF_NOTIFICATIONS_KEY, 0);

        SharedPreferences.Editor edit = sharedPreferences.edit();
        edit.putInt(NUMBER_OF_NOTIFICATIONS_KEY, id + 1);
        edit.commit();

        return id;
    }

    public long getTriggerTimeInMilliseconds(Integer hour24, Integer minute, Integer day) {
        int am_pm = Calendar.AM;
        if (hour24 >= 12) {
            am_pm = Calendar.PM;
        }

        int hour = hour24;
        if (hour24 > 12) {
            hour = hour24 - 12;
        }

        int HOURS_IN_A_WEEK = 168;

        Calendar currentCalendarTime = Calendar.getInstance();
        currentCalendarTime.setTimeInMillis(System.currentTimeMillis());

        Calendar calendarForReminder = Calendar.getInstance();
        calendarForReminder.setTimeInMillis(System.currentTimeMillis());
        calendarForReminder.set(Calendar.HOUR, hour);
        calendarForReminder.set(Calendar.MINUTE, minute);
        calendarForReminder.set(AM_PM, am_pm);
        calendarForReminder.set(DAY_OF_WEEK, day);

        if (calendarForReminder.before(currentCalendarTime)) {
            calendarForReminder.add(Calendar.HOUR, HOURS_IN_A_WEEK);
        }

        return calendarForReminder.getTimeInMillis();
    }

    private void createNotificationChannel(Context context) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            NotificationChannel channel1 = new NotificationChannel(
                    NOTIFICATION_CHANNEL_ID,
                    NOTIFICATION_CHANNEL_NAME,
                    NotificationManager.IMPORTANCE_HIGH
            );
            channel1.setDescription(NOTIFICATION_CHANNEL_DESCRIPTION);

            NotificationManager manager = context.getSystemService(NotificationManager.class);
            manager.createNotificationChannel(channel1);
        }
    }

    public static Bitmap drawableToBitmap(Drawable drawable) {

        if (drawable instanceof BitmapDrawable) {
            return ((BitmapDrawable) drawable).getBitmap();
        }

        Bitmap bitmap = Bitmap.createBitmap(
                drawable.getIntrinsicWidth(),
                drawable.getIntrinsicHeight(),
                Bitmap.Config.ARGB_8888);
        Canvas canvas = new Canvas(bitmap);
        drawable.setBounds(0, 0, canvas.getWidth(), canvas.getHeight());
        drawable.draw(canvas);

        return bitmap;
    }

    public Notification createReminderNotification() {
        createNotificationChannel(context);

        String title = "Positivelys";
        String message = "Create today's Positivelys!";

        Intent intent = new Intent(context, AppPageActivity.class);

        PendingIntent pendingIntent = PendingIntent.getActivity(context, NOTIFICATION_REQUEST_CODE, intent, 0);

        return new NotificationCompat.Builder(context, NOTIFICATION_CHANNEL_ID)
                .setSmallIcon(R.mipmap.ic_launcher_round)
                .setLargeIcon(drawableToBitmap(context.getResources().getDrawable(R.mipmap.ic_launcher, null)))
                .setContentTitle(title)
                .setContentText(message)
                .setPriority(NotificationCompat.PRIORITY_HIGH)
                .setCategory(NotificationCompat.CATEGORY_MESSAGE)
                .setContentIntent(pendingIntent)
                .build();
    }
}
